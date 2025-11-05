use crate::domain::entities::{User, UserId};
use crate::domain::value_objects::DisplayName;
use rand::Rng;

/// Domain service for generating random users
pub struct RandomUserService;

impl RandomUserService {
    pub fn generate_random_user() -> User {
        let names = vec![
            "Anonymous",
            "Wanderer",
            "Ghost",
            "Shadow",
            "Echo",
            "Phantom",
            "Whisper",
        ];
        let mut rng = rand::thread_rng();

        let name_index = rng.gen_range(0..names.len());
        let display_name = DisplayName::new(names[name_index].to_string());

        let seed = rng.gen_range(1..=100);
        let avatar_url = format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", seed);

        User::new(
            UserId(0), // ID will be set by repository
            display_name,
            avatar_url,
        )
    }
}
