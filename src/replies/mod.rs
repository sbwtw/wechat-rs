pub trait ReplyRenderer {
	fn render(&self) -> String;
}

mod text;
mod image;
mod voice;
mod video;
mod music;
mod transfer_customer_service;

pub use self::text::TextReply;
pub use self::image::ImageReply;
pub use self::voice::VoiceReply;
pub use self::video::VideoReply;
pub use self::music::MusicReply;
pub use self::transfer_customer_service::TransferCustomerServiceReply;

pub enum Reply {
    TextReply(TextReply),
    ImageReply(ImageReply),
    VoiceReply(VoiceReply),
    VideoReply(VideoReply),
    MusicReply(MusicReply),
    TransferCustomerServiceReply(TransferCustomerServiceReply),
}