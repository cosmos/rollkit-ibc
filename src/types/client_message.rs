use crate::types::Header;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientMessage {
	Header(Header),
	//Misbehaviour(Misbehaviour),
}