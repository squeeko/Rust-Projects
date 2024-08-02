// use anyhow::Result;
use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{Embedding, Linear, Module, VarBuilder};
use serde::Deserialize;

/*
// Creating a Tensor
let data: [u32; 3] = [1u32, 2, 3];
let tensor = Tensor::new(&data, &Device::Cpu);
println!("tensor: {:?}", tensor);

let nested_data: [[u32; 3]; 3] = [[1u32, 2, 3], [4, 5, 6], [7, 8, 9]];
let nested_tensor = Tensor::new(&nested_data, &Device::Cpu);
println!("nested_tensor: {:?}", nested_tensor);

println!(
    "Tensor Shape and Dim: {:?}",
    nested_tensor.expect("It Failed!").shape().dims()
);
*/

/*
Linear/Embedding: This is a helper function for loading the weights of a linear/embedding layer using VarBuilder from a checkpoint file. We create these 2 helper functions because we will use them multiple times.
*/
/*
A VarBuilder is used to retrieve variables used by a model. These variables can either come from a pre-trained checkpoint, e.g. using VarBuilder::from_mmaped_safetensors, or initialized for training, e.g. using VarBuilder::from_varmap.
 */

fn embedding(vocab_size: usize, hidden_size: usize, vb: VarBuilder) -> Result<Embedding> {
    let embeddings = vb.get((vocab_size, hidden_size), "weight")?;
    Ok(Embedding::new(embeddings, hidden_size))
}

fn linear(size1: usize, size2: usize, vb: VarBuilder) -> Result<Linear> {
    let weight = vb.get((size2, size1), "weight")?;
    let bias = vb.get(size2, "bias")?;
    Ok(Linear::new(weight, Some(bias)))
}

/*
https://www.pinecone.io/learn/batch-layer-normalization/#What-is-Layer-Normalization

Candle: In candle we can implement the layer normalization by creating or importing it directly from candle_nn with candle_nn::{LayerNorm,layer_norm} Steps:

Since normalization is done over the last axis which is the hidden size, we can use the sum_keepdim method to sum over the last axis and divide by dimension size to get mean_x.
For each element in the tensor, we subtract the mean and square the result and divide by hidden dimension to get norm_x.
To get the normalized input, we subtract the mean from the input and divide by the square root of norm_x + eps.
To get the final output, we multiply the normalized input by the weight of the normalization layer and add the bias.
 */

pub struct LayerNorm {
    weight: Tensor, // Weight vector of the LayerNorm Layer
    bias: Tensor,   // Bias vector of the LayerNorm Layer
    eps: f64,       // Epsilon value for numerical stability
}

impl LayerNorm {
    // Construction for LayerNorm
    pub fn new(weight: Tensor, bias: Tensor, eps: f64) -> Self {
        Self { weight, bias, eps }
    }

    pub fn forward(&self, x: Tensor) -> Result<Tensor> {
        let x_dtype = x.dtype(); // Get the data type of the input tensor
        let internal_dtype = match x_dtype {
            DType::F16 | DType::BF16 => DType::F32,
            d => d,
        };
        let (_bsize, _seq_len, hidden_size) = x.dims3()?; // Get the dimensions of the input tensor
        let x = x.to_dtype(internal_dtype)?;
        let mean_x = (x.sum_keepdim(2)? / hidden_size as f64)?; // Get the mean of the input tensor and divide by the hidden size
        let x = x.broadcast_sub(&mean_x)?; // Subtract the mean from the input tensor
        let norm_x = (x.sqr()?.sum_keepdim(2)? / hidden_size as f64)?; // Get the squared norm of the input tensor and divide by the hidden size
        let x_normed = x.broadcast_div(&(norm_x + self.eps)?.sqrt()?)?; // Get the normalized input
        let x = x_normed
            .to_dtype(x_dtype)?
            .broadcast_mul(&self.weight)?
            .broadcast_add(&self.bias)?;
        Ok(x)
    }
}

// Dropout Layer: Randomly zero out different parts of the input tensor using a probability value. This is only used during training!!!!!
// When using pretrained models this is NOT needed!

struct Dropout {
    #[allow(dead_code)]
    pr: f64,
}

impl Dropout {
    fn new(pr: f64) -> Self {
        Self { pr }
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        Ok(x.clone())
    }
}

/*
RoBERTa Configuration - This is a struct that holds the configuration of the model. It is similar to the RobertaConfig in the transformers library. For this Struct, We will initialize the default values for the config (We implement the Default trait for the RobertaConfig struct ) and then use the serde crate to deserialize the config from a json file. Alternatively we can create a RobertaConfig::new() method for creating a new instance of RobertaConfig. See the paramters to be created from this link

https://github.com/huggingface/transformers/blob/e1cec43415e72c9853288d4e9325b734d36dd617/src/transformers/models/roberta/configuration_roberta.py#L37

*/

pub struct RobertaConfig {
    vocab_size: usize,
    hidden_size: usize,
    num_hidden_layers: usize,
    num_attention_heads: usize,
    intermediate_size: usize,
    hidden_act: String,
    hidden_dropout_prob: f64,
    max_position_embeddings: usize,
    type_vocab_size: usize,
    initializer_range: f64,
    layer_norm_eps: f64,
    pad_token_id: usize,
    bos_token_id: usize,
    eos_token_id: usize,
    position_embedding_type: String,
    use_cache: bool,
    classifier_dropout: Option<f64>,
    model_type: Option<String>,
}

impl Default for RobertaConfig {
    fn default() -> Self {
        Self {
            vocab_size: 50265,
            hidden_size: 768,
            num_hidden_layers: 12,
            num_attention_heads: 12,
            intermediate_size: 3072,
            hidden_act: "gelu".to_string(),
            hidden_dropout_prob: 0.1,
            max_position_embeddings: 512,
            type_vocab_size: 2,
            initializer_range: 0.02,
            layer_norm_eps: 1e-12,
            pad_token_id: 1,
            bos_token_id: 0,
            eos_token_id: 2,
            position_embedding_type: "Absolute".to_string(),
            use_cache: true,
            classifier_dropout: None,
            model_type: Some("roberta".to_string()),
        }
    }
}

pub fn create_position_ids_from_input_embeds(&self, input_embeds: &Tensor) -> Result<Tensor> {
    // input_shape = inputs_embeds.size()
    // In candle, we use dims3() for getting the size of a 3 dimensional tensor
    let input_shape = input_embeds.dims3()?;
    // sequence_length = input_shape[1]
    let seq_length = input_shape.1;

    // position_ids = torch.arange( self.padding_idx + 1, sequence_length + self.padding_idx + 1, \
    // dtype=torch.long, device=inputs_embeds.device)
    let mut position_ids = Tensor::arange(
        self.padding_idx + 1,
        seq_length as u32 + self.padding_idx + 1,
        &Device::Cpu,
    )?;

    // return position_ids.unsqueeze(0).expand(input_shape)
    position_ids = position_ids
        .unsqueeze(0)?
        .expand((input_shape.0, input_shape.1))?;
    Ok(position_ids)
}

pub fn create_position_ids_from_input_ids(
    input_ids: &Tensor,
    padding_idx: u32,
    past_key_values_length: u8,
) -> Result<Tensor> {
    // mask = input_ids.ne(padding_idx).int()
    let mask = input_ids.ne(padding_idx)?;
    // incremental_indices = (torch.cumsum(mask, dim=1).type_as(mask) + past_key_values_length) * mask
    let incremental_indices = cumsum_2d(&mask, 0, input_ids.device())?;

    // incremental_indices.long() + padding_idx
    let incremental_indices = incremental_indices
        .broadcast_add(&Tensor::new(&[past_key_values_length], input_ids.device())?)?;

    Ok(incremental_indices)
}

pub struct RobertaEmbeddings {
    word_embeddings: Embedding,
    position_embeddings: Option<Embedding>,
    token_type_embeddings: Embedding,
    layer_norm: LayerNorm,
    dropout: Dropout,
    pub padding_idx: u32,
}

impl RobertaEmbeddings {
    pub fn load(vb: VarBuilder, config: &RobertaConfig) -> Result<Self> {
        // nn.Embedding(config.vocab_size, config.hidden_size)
        let word_embeddings = embedding(
            config.vocab_size,
            config.hidden_size,
            vb.pp("word_embeddings"),
        )?;

        // nn.Embedding(config.max_position_embeddings, config.hidden_size)
        let position_embeddings = embedding(
            config.max_position_embeddings,
            config.hidden_size,
            vb.pp("position_embeddings"),
        )?;

        // nn.Embedding(config.type_vocab_size, config.hidden_size)
        let token_type_embeddings = embedding(
            config.type_vocab_size,
            config.hidden_size,
            vb.pp("token_type_embeddings"),
        )?;

        // nn.LayerNorm(config.hidden_size, eps=config.layer_norm_eps)
        let layer_norm = layer_norm(
            config.hidden_size,
            config.layer_norm_eps,
            vb.pp("LayerNorm"),
        )?;

        // nn.Dropout(config.hidden_dropout_prob)
        let dropout = Dropout::new(config.hidden_dropout_prob);

        let padding_idx = config.pad_token_id as u32;

        Ok(Self {
            word_embeddings,
            position_embeddings: Some(position_embeddings),
            token_type_embeddings,
            layer_norm,
            dropout,
            padding_idx,
        })
    }

    pub fn forward(
        &self,
        input_ids: &Tensor,
        token_type_ids: &Tensor,
        position_ids: Option<&Tensor>,
        inputs_embeds: Option<&Tensor>,
    ) -> Result<Tensor> {
        let position_ids = match position_ids {
            Some(ids) => ids.to_owned(),
            None => {
                if Option::is_some(&inputs_embeds) {
                    // self.create_position_ids_from_inputs_embeds(inputs_embeds)
                    let position_ids =
                        self.create_position_ids_from_input_embeds(inputs_embeds.unwrap())?; //
                    position_ids
                } else {
                    // create_position_ids_from_input_ids(input_ids, self.padding_idx, past_key_values_length)
                    let position_ids =
                        create_position_ids_from_input_ids(input_ids, self.padding_idx, 1)?;
                    position_ids
                }
            }
        };

        let inputs_embeds: Tensor = match inputs_embeds {
            Some(embeds) => embeds.to_owned(),
            None => {
                // self.word_embeddings(input_ids)
                let embeds = self.word_embeddings.forward(input_ids)?;
                embeds
            }
        };

        // self.token_type_embeddings(token_type_ids)
        let token_type_embeddings = self.token_type_embeddings.forward(token_type_ids)?;
        // inputs_embeds + token_type_embeddings
        let mut embeddings = (inputs_embeds + token_type_embeddings)?;

        if let Some(position_embeddings) = &self.position_embeddings {
            // embeddings + self.position_embeddings(position_ids)
            embeddings = embeddings.broadcast_add(&position_embeddings.forward(&position_ids)?)?
        }

        // self.LayerNorm(embeddings)
        let embeddings = self.layer_norm.forward(&embeddings)?;
        // self.dropout(embeddings)
        let embeddings = self.dropout.forward(&embeddings)?;

        Ok(embeddings)
    }
}

fn main() {
    // This struct can be used as follows
    let w_gen = Tensor::new(&[[3f32, 1.]], &Device::Cpu).expect("Weight Input Error");
    let b_gen = Tensor::new(-2f32, &Device::Cpu).expect("Bias Input Error");

    // Initialize a layer norn layer
    let layer_norm = LayerNorm::new(w_gen, b_gen, 1f64);

    let data: [u32; 3] = [1u32, 2, 3];
    dbg!(&data);
    let input_tensor = Tensor::new(&data, &Device::Cpu).expect("Input Tensor Error");
    let normalized_tensor = layer_norm
        .forward(input_tensor)
        .expect("Normalized Tensor Error");

    // Dropout struct to be used as follows
    /*
    let dropout = Dropout::new(0.1);

    let data: [u32; 3] = [1u32, 2, 3];
    let input_tensor = Tensor::new(&data, &Device::Cpu)?;
    let dropout_tensor = dropout.forward(&input_tensor)?;
    */
}
