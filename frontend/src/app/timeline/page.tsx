'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/lib/auth-context';
import { CreatePost } from '@/components/CreatePost';
import { Timeline } from '@/components/Timeline';
import { Button } from '@/components/ui/button';
import { ReactionNotification } from '@/components/ReactionNotification';

export default function TimelinePage() {
	const { isAuthenticated, logout } = useAuth();
	const router = useRouter();

	useEffect(() => {
		if (!isAuthenticated) {
			router.push('/login');
		}
	}, [isAuthenticated, router]);

	const handleLogout = async () => {
		await logout();
		router.push('/login');
	};

	if (!isAuthenticated) {
		return null;
	}

	return (
		<main className="min-h-screen relative overflow-hidden">
			{/* Subtle background effect */}
			<div className="fixed inset-0 opacity-5 pointer-events-none">
				<div className="absolute top-20 left-20 w-96 h-96 bg-blue-400 rounded-full blur-[120px]" />
				<div className="absolute bottom-20 right-20 w-96 h-96 bg-pink-400 rounded-full blur-[120px]" />
			</div>

			{/* Real-time reaction notifications via SSE */}
			<ReactionNotification />

			{/* Floating action button for creating posts */}
			<CreatePost />

			<div className="container mx-auto px-4 py-8 max-w-3xl relative z-10">
				<header className="mb-12 flex justify-between items-center">
					<h1 className="text-5xl font-bold gradient-text">
						Echo
					</h1>
					<div className="flex gap-3">
						<Button
							variant="outline"
							onClick={() => router.push('/avatar')}
							className="hover:bg-blue-500/5 transition-all duration-300"
						>
							マスコット
						</Button>
						<Button
							variant="outline"
							onClick={handleLogout}
							className="hover:bg-pink-500/5 transition-all duration-300"
						>
							ログアウト
						</Button>
					</div>
				</header>

				<Timeline />
			</div>

		</main>
	);
}
