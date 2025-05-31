use actix::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type Db = HashMap<String, Vec<String>>;

#[derive(Clone)]
pub struct ActorWCM {
    word_in_context: Arc<Mutex<Db>>, 
}
impl ActorWCM {
    pub fn new() -> Self {
        ActorWCM {
            word_in_context: Arc::new(Mutex::new(Db::new())),
        }
    }
}
impl Actor for ActorWCM {
    type Context = Context<Self>;
}

// ===== Ping =====
// - Verifica se o ator está funcionando.
// - Imprime mensagem no console.
#[derive(Message)]
#[rtype(result = "bool")]
pub struct Ping();
impl Handler<Ping> for ActorWCM {
    type Result = bool;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor {} ping!", "WCM");
        true
    }
}

// ===== ReqWIC =====
// - Envia o HashMap atual.
#[derive(Message)]
#[rtype(result = "Result<Db, std::io::Error>")]
pub struct ReqWIC { }
impl ReqWIC {
    pub fn new() -> Self {
        ReqWIC { }
    }
}
impl Handler<ReqWIC> for ActorWCM {
    type Result = ResponseFuture<Result<Db, std::io::Error>>;

    fn handle(&mut self, _msg: ReqWIC, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();
        
        Box::pin(async move {
            let guard = this.word_in_context
            .lock() // Inicia aquisição do lock → Future
            .await; // Aguarda disponibilidade do lock

            Ok(guard.clone()) // Clona os dados (fora do statement que contém o lock, para evitar o lock travado) e retorna eles no Future
        })
    }
}


// ===== KeywordAdd =====
// - Adiciona uma nova frase em keyword
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct KeywordAdd {
    key: String,
    sentence: Arc<String>,
}
impl KeywordAdd {
    pub fn new(key: String, sentence: Arc<String>) -> Self {
        KeywordAdd {
            key,
            sentence,
        }
    }
}
impl Handler<KeywordAdd> for ActorWCM {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: KeywordAdd, _ctx: &mut Context<Self>) -> Self::Result {
        let SIZE_WIC_AROUND: usize = 2; // Transformar em não-hardcoded eventualmente
        let parts = msg.sentence.split(" ");
        let word_list = parts.collect::<Vec<&str>>();
        
        let key: String = msg.key.clone().to_lowercase();
        // println!("KEYWORDADD: {}", key);
        let mut i_pos = 0;
        for (i, word) in word_list.iter().enumerate() {
            // TODO: cheque sempre lowercase
            if *word.to_lowercase() == key {
                i_pos = i;
                break;
            }
        }

        let value: String;
        if word_list.len() <= SIZE_WIC_AROUND*2 + 1 {
            // Todas palavras são inlusas, a frase
            // original é muito pequena (5 ou menos
            // palavras no caso padrão)
            let mut result = "".to_owned();
            result.push_str(&msg.key);
            result.push_str(" ");
            for word in word_list.iter().skip(i_pos+1) {
                result.push_str(word);
                result.push_str(" ");
            }
            for word in word_list.iter() {
                if word.to_lowercase() == key {
                    break;
                }
                result.push_str(word);
                result.push_str(" ");
            }
            value = result.trim().to_string();
        } else {
            // Frase maior do que 5 palavras (no padrão)
            // (algumas palavras são excluídas do contexto)
            let mut result = "".to_owned();
            result.push_str(&msg.key);
            result.push_str(" ");

            let mut i_bef: isize = i_pos.try_into().unwrap();
            i_bef -= <usize as TryInto<isize>>::try_into(SIZE_WIC_AROUND).unwrap();
            let i_aft: usize = i_pos+SIZE_WIC_AROUND;
            if i_aft < word_list.len() {
                let mut w_amnt: usize = 0;
                for (i, word) in word_list.iter().skip(i_pos+1).enumerate() {
                    if i == i_aft+1 || w_amnt == SIZE_WIC_AROUND {
                        break;
                    }
                    result.push_str(word);
                    result.push_str(" ");
                    w_amnt += 1;
                }
            } else {
                let mut w_amnt: usize = 0;
                
                for word in word_list.iter().skip(i_pos+1) {
                    result.push_str(word);
                    result.push_str(" ");
                    w_amnt += 1;
                }
                
                for word in word_list.iter() {
                    if w_amnt == SIZE_WIC_AROUND {
                        break;
                    }
                    result.push_str(word);
                    result.push_str(" ");
                    w_amnt += 1;
                }

            }
            result = result.trim().to_string();
            result.push_str(" ... ");

            if i_bef >= 0 {
                let mut w_amnt: usize = 0;
                for (i, word) in word_list.iter().skip(i_bef.try_into().unwrap()).enumerate() {
                    if i == i_pos || w_amnt == SIZE_WIC_AROUND {
                        break;
                    }
                    result.push_str(word);
                    result.push_str(" ");
                    w_amnt += 1;
                }
            } else {
                let mut w_amnt: usize = 0;
                let end_skip = word_list.len() - <isize as TryInto<usize>>::try_into(i_bef.abs()).unwrap();

                for word in word_list.iter().skip(end_skip) {
                    result.push_str(word);
                    result.push_str(" ");
                    w_amnt += 1;
                }
                for (i, word) in word_list.iter().enumerate() {
                    if i == i_pos || w_amnt == SIZE_WIC_AROUND {
                        break;
                    }
                    result.push_str(word);
                    result.push_str(" ");
                    w_amnt += 1;
                }
            }

            value = result.trim().to_string();
        }
        // println!("VALUE: {}", value);

        let this = self.clone();
        Box::pin(async move {
            let mut wic_dict = this.word_in_context.lock().await;
            if wic_dict.contains_key(&key) {
                let mut new_value: Vec<String> = wic_dict.get(&key).unwrap().to_vec();
                new_value.push(value.to_string());
                
                // debug
                // for deb in new_value.iter() {
                //     println!("{}: {}", key, deb);
                // }

                wic_dict.insert(key, new_value);

            } else {
                let mut new_value: Vec<String> = Vec::new();
                new_value.push(value.to_string());
                
                // debug
                // for deb in new_value.iter() {
                //     println!("{}: {}", key, deb);
                // }

                wic_dict.insert(key, new_value);
            }
            
            Ok(true)
        })
    }
}