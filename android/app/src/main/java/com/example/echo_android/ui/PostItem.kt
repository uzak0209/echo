package com.example.echo_android.ui

import android.R.attr.text
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
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color

enum class ReactionType(val emoji: String) {
    SURPRISE("😮"),
    EMPATHY("❤️"),
    LAUGH("😂"),
    SAD("😢"),
    CONFUSED("😕")
}

data class  Post(
    val id: Int,
    val username: String,
    val text: String,
    val reactions: Map<String, Reaction>,
)

data class Reaction(
    var count: Int,
    var active: Boolean,
    val type: ReactionType,
)

@Composable
fun PostItem(post: Post) {
    var reactions by remember { mutableStateOf(post.reactions) }

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(8.dp),
        horizontalArrangement = Arrangement.Center)
    {
        // icon
        Box(
            modifier = Modifier
                .size(48.dp)
                .clip(CircleShape)
                .background(color = Color.Gray),
            contentAlignment = Alignment.Center
        ) {

        }

        Spacer(modifier = Modifier.width(8.dp))

        Column(modifier = Modifier.fillMaxWidth()) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(text = post.username)
                Spacer(modifier = Modifier.size(4.dp))
                Text(text = "@" + post.id.toString())
            }

            Spacer(modifier = Modifier.size(4.dp))

            Card(
                modifier = Modifier.fillMaxWidth()
            ) {
                Column(modifier = Modifier.padding(8.dp)) {
                    Text(text = post.text)
                }
            }

            Spacer(modifier = Modifier.size(4.dp))

            // リアクションボタン
            Row(
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                ReactionType.entries.forEach { reactionType ->
                    ReactionButton(
                        reaction = reactionType,
                        onClick = { /* TODO: リアクション*/}
                    )
                }

            }
        }
    }

}

@Composable
fun ReactionButton(reaction: ReactionType, onClick: () -> Unit = {}) {
    Button(
        onClick = onClick
    ) {
        Text(
            text = reaction.emoji
        )
    }
}

@Preview(showBackground = true)
@Composable
fun PostItemPreview() {
    PostItem(
        post = Post(
            id = 1,
            username = "kun",
            text = "aaaaaa",
            reactions = emptyMap()
        )
    )
}