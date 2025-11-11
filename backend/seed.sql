-- Seed data for Echo app
-- This script creates test users and posts for development

-- Clear existing data (optional - be careful in production!)
TRUNCATE TABLE reactions CASCADE;
TRUNCATE TABLE posts CASCADE;
TRUNCATE TABLE users CASCADE;

-- Insert test users
-- Password for all users is 'password123'
-- bcrypt hash of 'password123': $2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyB9Z7aGPq0u

INSERT INTO users (id, display_name, avatar_url, password_hash, valid, created_at, refresh_token) VALUES
('00000000-0000-0000-0000-000000000001', 'alice', 'https://api.dicebear.com/7.x/avataaars/svg?seed=alice', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyB9Z7aGPq0u', true, NOW(), NULL),
('00000000-0000-0000-0000-000000000002', 'bob', 'https://api.dicebear.com/7.x/avataaars/svg?seed=bob', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyB9Z7aGPq0u', true, NOW(), NULL),
('00000000-0000-0000-0000-000000000003', 'charlie', 'https://api.dicebear.com/7.x/avataaars/svg?seed=charlie', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyB9Z7aGPq0u', true, NOW(), NULL),
('00000000-0000-0000-0000-000000000004', 'diana', 'https://api.dicebear.com/7.x/avataaars/svg?seed=diana', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyB9Z7aGPq0u', true, NOW(), NULL),
('00000000-0000-0000-0000-000000000005', 'eve', 'https://api.dicebear.com/7.x/avataaars/svg?seed=eve', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyB9Z7aGPq0u', true, NOW(), NULL);

-- Insert test posts
INSERT INTO posts (id, user_id, content, image_url, display_count, valid, created_at) VALUES
('10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'はじめての投稿です！よろしくお願いします 🎉', NULL, 0, true, NOW() - INTERVAL '5 days'),
('10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'こんにちは！今日は良い天気ですね ☀️', NULL, 1, true, NOW() - INTERVAL '4 days'),
('10000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'ランチに美味しいラーメンを食べました 🍜', NULL, 2, true, NOW() - INTERVAL '3 days'),
('10000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000004', 'コーディング楽しい！新しい技術を学んでいます 💻', NULL, 0, true, NOW() - INTERVAL '2 days'),
('10000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000005', '散歩してきました。気分転換になりました 🚶', NULL, 3, true, NOW() - INTERVAL '1 day'),
('10000000-0000-0000-0000-000000000006', '00000000-0000-0000-0000-000000000001', 'プログラミングの勉強頑張ってます！', NULL, 1, true, NOW() - INTERVAL '12 hours'),
('10000000-0000-0000-0000-000000000007', '00000000-0000-0000-0000-000000000002', 'カフェで読書中 ☕📖', NULL, 0, true, NOW() - INTERVAL '6 hours'),
('10000000-0000-0000-0000-000000000008', '00000000-0000-0000-0000-000000000003', '今日のランチは和食でした 🍱', NULL, 2, true, NOW() - INTERVAL '3 hours'),
('10000000-0000-0000-0000-000000000009', '00000000-0000-0000-0000-000000000004', 'ジムでトレーニングしてきました 💪', NULL, 1, true, NOW() - INTERVAL '1 hour'),
('10000000-0000-0000-0000-000000000010', '00000000-0000-0000-0000-000000000005', 'このアプリ、使いやすいですね！', NULL, 0, true, NOW() - INTERVAL '30 minutes');

-- Insert some reactions
INSERT INTO reactions (id, post_id, user_id, reaction_type, created_at) VALUES
('20000000-0000-0000-0000-000000000001', '10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000002', 'like', NOW() - INTERVAL '4 days'),
('20000000-0000-0000-0000-000000000002', '10000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000003', 'love', NOW() - INTERVAL '4 days'),
('20000000-0000-0000-0000-000000000003', '10000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 'like', NOW() - INTERVAL '3 days'),
('20000000-0000-0000-0000-000000000004', '10000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000004', 'laugh', NOW() - INTERVAL '2 days'),
('20000000-0000-0000-0000-000000000005', '10000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000001', 'like', NOW() - INTERVAL '1 day');

-- Show summary
SELECT 'Users created:' as info, COUNT(*) as count FROM users
UNION ALL
SELECT 'Posts created:', COUNT(*) FROM posts
UNION ALL
SELECT 'Reactions created:', COUNT(*) FROM reactions;

-- Show test credentials
SELECT '=== Test User Credentials ===' as info
UNION ALL
SELECT 'Username: alice, Password: password123'
UNION ALL
SELECT 'Username: bob, Password: password123'
UNION ALL
SELECT 'Username: charlie, Password: password123'
UNION ALL
SELECT 'Username: diana, Password: password123'
UNION ALL
SELECT 'Username: eve, Password: password123';
