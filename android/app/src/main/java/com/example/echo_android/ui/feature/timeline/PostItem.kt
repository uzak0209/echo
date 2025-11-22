package com.example.echo_android.ui.feature.timeline

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.FilledIconButton
import androidx.compose.material3.IconButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.example.echo_android.R
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.type.ReactionTypeGql

@Composable
fun PostItem(
    post: GetTimelineQuery.Timeline,
    userReaction: ReactionTypeGql?,
    onReactionClick: (postId: String, reactionType: ReactionTypeGql, isActive: Boolean) -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surface
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp)
        ) {
            // アバター
            AsyncImage(
                model = post.authorAvatar,
                contentDescription = "Author icon",
                modifier = Modifier
                    .size(40.dp)
                    .clip(CircleShape)
                    .background(color = MaterialTheme.colorScheme.surfaceVariant),
                placeholder = painterResource(R.drawable.ic_launcher_foreground),
                error = painterResource(R.drawable.ic_launcher_foreground),
                contentScale = ContentScale.Crop
            )

            Spacer(modifier = Modifier.width(12.dp))

            Column(modifier = Modifier.weight(1f)) {
                // ユーザー名
                Text(
                    text = post.authorName,
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface
                )

                Spacer(modifier = Modifier.height(8.dp))

                // 投稿内容(吹き出し風カード)
                Card(
                    modifier = Modifier.fillMaxWidth(),
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.surfaceContainerHighest
                    ),
                    shape = MaterialTheme.shapes.medium
                ) {
                    Text(
                        text = post.content,
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurface,
                        modifier = Modifier.padding(12.dp)
                    )
                }

                Spacer(modifier = Modifier.height(12.dp))

                // リアクションボタン
                Row(
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    modifier = Modifier.fillMaxWidth()
                ) {
                    ReactionTypeGql.entries
                        .filter { it != ReactionTypeGql.UNKNOWN__ }
                        .forEach { reactionType ->
                            val isActive = userReaction == reactionType

                            ReactionButton(
                                reaction = reactionType,
                                active = isActive,
                                onClick = {
                                    onReactionClick(post.id, reactionType, isActive)
                                }
                            )
                        }
                }
            }

        }
    }
}

@Composable
fun ReactionButton(
    reaction: ReactionTypeGql,
    active: Boolean = false,
    onClick: () -> Unit
) {
    FilledIconButton(
        onClick = onClick,
        modifier = Modifier.size(40.dp),
        colors = IconButtonDefaults.filledIconButtonColors(
            containerColor = if (active) {
                MaterialTheme.colorScheme.primaryContainer
            } else {
                MaterialTheme.colorScheme.surfaceVariant
            },
            contentColor = if (active) {
                MaterialTheme.colorScheme.onPrimaryContainer
            } else {
                MaterialTheme.colorScheme.onSurfaceVariant
            }
        )
    ) {
        Text(
            text = when (reaction) {
                ReactionTypeGql.SURPRISE -> "😲"
                ReactionTypeGql.EMPATHY -> "🥺"
                ReactionTypeGql.LAUGH -> "😄"
                ReactionTypeGql.SAD -> "😢"
                ReactionTypeGql.CONFUSED -> "😕"
                else -> "???"
            },
            style = MaterialTheme.typography.titleMedium
        )
    }
}

@Preview(showBackground = true)
@Composable
fun PostItemPreview() {
    MaterialTheme {
        PostItem(
            post = GetTimelineQuery.Timeline(
                id = "1",
                authorName = "Hibiki",
                authorAvatar = "https://pbs.twimg.com/profile_images/1534646026870507520/8b4n9_2Q_400x400.jpg",
                content = "Hello, World!",
                imageUrl = "",
            ),
            userReaction = ReactionTypeGql.SURPRISE,
            onReactionClick = { _, _, _ -> }
        )
    }
}