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

	if (!isAuthenticated) {
		return null;
	}

	return (
		<main className="min-h-screen bg-background">
			{/* Real-time reaction notifications via SSE */}
			<ReactionNotification />

			<div className="container mx-auto px-4 py-8 max-w-2xl">
				<header className="mb-8 flex justify-between items-center">
					<div>
						<h1 className="text-4xl font-bold mb-2">Echo</h1>
					</div>
					<div className="flex gap-2">
						<CreatePost />
						<Button variant="outline" onClick={() => router.push('/avatar')}>
							Mascot
						</Button>
						<Button variant="outline" onClick={logout}>
							Logout
						</Button>
					</div>
				</header>

				<Timeline />
			</div>
		</main>
	);
}
