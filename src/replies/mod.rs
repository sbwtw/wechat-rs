pub trait ReplyRenderer {
	fn render(&self) -> String;
}

mod text;
mod image;
mod voice;
mod transfer_customer_service;

pub use self::text::TextReply;
pub use self::image::ImageReply;
pub use self::voice::VoiceReply;
pub use self::transfer_customer_service::TransferCustomerServiceReply;

pub enum Reply {
    TextReply(TextReply),
    ImageReply(ImageReply),
    VoiceReply(VoiceReply),
    TransferCustomerServiceReply(TransferCustomerServiceReply),
}