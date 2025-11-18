package com.example.echo_android.ui.feature.timeline

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.compose.ui.Alignment
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.painter.Painter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import com.example.echo_android.R
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.type.ReactionTypeGql
import coil.compose.AsyncImage

@Composable
fun PostItem(
    post: GetTimelineQuery.Timeline,
    userReaction: ReactionTypeGql?,
    onReactionClick: (postId: String, reactionType: ReactionTypeGql, isActive: Boolean) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(8.dp),
        horizontalArrangement = Arrangement.Center)
    {
        // icon
        AsyncImage(
            model = post.authorAvatar.also {
                android.util.Log.d("PostItem", "Loading image URL: $it")
            },
            contentDescription = "Author icon",
            modifier = Modifier
                .size(48.dp)
                .clip(CircleShape)
                .background(color = MaterialTheme.colorScheme.surface),
            placeholder = painterResource(R.drawable.ic_launcher_foreground), // ロード中の表示
            error = painterResource(R.drawable.ic_launcher_foreground), // エラー時の表示
            contentScale = ContentScale.Crop
        )

        Spacer(modifier = Modifier.width(8.dp))

        Column(modifier = Modifier.fillMaxWidth()) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(text = post.authorName)
                Spacer(modifier = Modifier.size(4.dp))
            }

            Spacer(modifier = Modifier.size(4.dp))

            Card(
                modifier = Modifier.fillMaxWidth()
            ) {
                Column(modifier = Modifier.padding(8.dp)) {
                    Text(text = post.content)
                }
            }

            Spacer(modifier = Modifier.size(4.dp))

            // リアクションボタン
            Row(horizontalArrangement = Arrangement.SpaceEvenly) {
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

@Composable
fun ReactionButton(
    reaction: ReactionTypeGql,
    active: Boolean = false,
    onClick: () -> Unit
) {
    IconButton(
        onClick = onClick,
        modifier = Modifier
            .background(
                // TODO: Update color scheme later
                color = if (active) {
                    MaterialTheme.colorScheme.primaryContainer // アクティブ時
                } else {
                    Color.Transparent // 非アクティブ時
                },
        shape = CircleShape
    )
    ) {
        Text(
            text = when (reaction) {
                ReactionTypeGql.SURPRISE -> "😲"
                ReactionTypeGql.EMPATHY -> "🥺"
                ReactionTypeGql.LAUGH -> "😂"
                ReactionTypeGql.SAD -> "😢"
                ReactionTypeGql.CONFUSED -> "😕"
                else -> "???"
            }
        )
    }
}

//@Preview(showBackground = true)
//@Composable
//fun PostItemPreview() {
//    PostItem(
//        post = Post(
//            id = 1,
//            username = "kun",
//            text = "aaaaaa",
//            reactions = emptyMap()
//        )
//    )
//}