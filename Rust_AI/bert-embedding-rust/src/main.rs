// Figure out the Dropout package....

/*
Before we delve into BERT’s embedding layer, we define a Config struct that will hold all our model configurations. This Config struct is a direct implementation of the BertConfig Python class with default values for a bert-base-uncased model.
*/

use std::borrow::Borrow;
use tch::nn;
use tch::Kind;
use tch::Tensor;

pub struct Config {
    pub vocab_size: i64,
    pub hidden_size: i64,
    pub num_hidden_layers: i64,
    pub num_attention_heads: i64,
    pub intermediate_size: i64,
    pub hidden_act: String,
    pub hidden_dropout_prob: f64,
    pub attention_probs_dropout_prob: f64,
    pub max_position_embeddings: i64,
    pub type_vocab_size: i64,
    pub initializer_range: f64,
    pub layer_norm_eps: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vocab_size: 30522,
            hidden_size: 768,
            num_hidden_layers: 12,
            num_attention_heads: 12,
            intermediate_size: 3072,
            hidden_act: "gelu".to_string(),
            hidden_dropout_prob: 0.1,
            attention_probs_dropout_prob: 0.1,
            max_position_embeddings: 512,
            type_vocab_size: 2,
            initializer_range: 0.02,
            layer_norm_eps: 1e-12,
        }
    }
}

/*
Implementing BERT Embeddings in Rust
The BertEmbeddings struct, just like the BertEmbeddings class in the Transformers library has an initialization method and the classic PyTorch forward method. This method takes in the input Tensor from the tokenizer and passes it through the model.

Each field in BertEmbeddings has a specific role:

word_embeddings: An nn::Embedding layer, which maps input tokens to their embeddings.
position_embeddings: An embedding layer for the input positions.
token_type_embeddings: An embedding layer for the input token types.
layer_norm: A layer normalization layer.
dropout: A dropout layer.
*/

pub struct BertEmbeddings {
    word_embeddings: nn::Embedding,
    position_embeddings: nn::Embedding,
    token_type_embeddings: nn::Embedding,
    layer_norm: nn::LayerNorm,
    dropout: Dropout,
}

impl BertEmbeddings {
    pub fn new<'p, P>(p: P, config: &Config) -> BertEmbeddings
    where
        P: Borrow<nn::Path<'p>>,
    {
        let p = p.borrow();
        let word_embeddings = nn::embedding(
            p / "word_embeddings",
            config.vocab_size,
            config.hidden_size,
            Default::default(),
        );
        let position_embeddings = nn::embedding(
            p / "position_embeddings",
            config.max_position_embeddings,
            config.hidden_size,
            Default::default(),
        );
        let token_type_embeddings = nn::embedding(
            p / "token_type_embeddings",
            config.type_vocab_size,
            config.hidden_size,
            Default::default(),
        );
        let layer_norm_config = nn::LayerNormConfig {
            eps: config.layer_norm_eps,
            ..Default::default()
        };
        let layer_norm =
            nn::layer_norm(p / "LayerNorm", vec![config.hidden_size], layer_norm_config);
        let dropout = Dropout::new(config.hidden_dropout_prob);
        Self {
            word_embeddings,
            position_embeddings,
            token_type_embeddings,
            layer_norm,
            dropout,
        }
    }

    pub fn forward_t(
        &self,
        input_ids: &Tensor,
        token_type_ids: Option<&Tensor>,
        position_ids: Option<&Tensor>,
        train: bool,
    ) -> Result<Tensor, &'static str> {
        let input_shape = input_ids.size();
        let seq_length = input_shape[1];
        let device = input_ids.device();
        let input_ids = input_ids.view((-1, seq_length));
        let position_ids = match position_ids {
            Some(position_ids) => position_ids.view((-1, seq_length)),
            None => Tensor::arange(seq_length, (Kind::Int64, device))
                .unsqueeze(0)
                .expand(&input_shape, true),
        };
        let token_type_ids = match token_type_ids {
            Some(token_type_ids) => token_type_ids.view((-1, seq_length)),
            None => Tensor::zeros(&input_shape, (Kind::Int64, device)),
        };

        let word_embeddings = input_ids.apply(&self.word_embeddings);
        let position_embeddings = position_ids.apply(&self.position_embeddings);
        let token_type_embeddings = token_type_ids.apply(&self.token_type_embeddings);
        let embeddings = word_embeddings + position_embeddings + token_type_embeddings;

        Ok(embeddings
            .apply(&self.layer_norm)
            .apply_t(&self.dropout, train))
    }
}

fn main() {
    println!("Hello, world!");
}
