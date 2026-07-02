use uuid::Uuid;

pub fn generate() -> String {
    format!(
        "sk_live_{}",
        Uuid::new_v4()
            .simple()
    )
}