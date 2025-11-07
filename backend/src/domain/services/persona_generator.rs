use rand::Rng;

/// ランダムアバターを生成するサービス
pub struct PersonaGenerator;

impl PersonaGenerator {
    const AVATAR_COLORS: &'static [&'static str] = &[
        "1e40af", "7c3aed", "db2777", "dc2626", "ea580c",
        "d97706", "65a30d", "059669", "0891b2", "0284c7",
        "4f46e5", "7c2d12", "be123c", "9f1239", "a21caf",
    ];

    /// ランダムなアバターURLを生成（DiceBear API使用）
    pub fn generate_avatar() -> String {
        let mut rng = rand::thread_rng();
        let seed = uuid::Uuid::new_v4().to_string();
        let color = Self::AVATAR_COLORS[rng.gen_range(0..Self::AVATAR_COLORS.len())];

        // DiceBear Avatars API - Identicon style
        format!(
            "https://api.dicebear.com/7.x/identicon/svg?seed={}&backgroundColor={}",
            seed, color
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_avatar() {
        let avatar = PersonaGenerator::generate_avatar();
        assert!(avatar.starts_with("https://api.dicebear.com"));
        println!("Generated avatar: {}", avatar);
    }

    #[test]
    fn test_multiple_generations_are_different() {
        let avatar1 = PersonaGenerator::generate_avatar();
        let avatar2 = PersonaGenerator::generate_avatar();

        // 高確率で異なる値が生成される
        assert_ne!(avatar1, avatar2);

        println!("Avatar 1: {}", avatar1);
        println!("Avatar 2: {}", avatar2);
    }
}
