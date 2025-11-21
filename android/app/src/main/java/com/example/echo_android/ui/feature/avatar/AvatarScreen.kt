package com.example.echo_android.ui.feature.avatar

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun AvatarScreen(
    expression: String? = null
) {
    // TODO: MainViewModelが追加されたら、viewModel.avatarExpression.collectAsState()に戻す
    val avatarExpression = expression

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        AvatarDisplay(
            expression = avatarExpression ?: "neutral",
            modifier = Modifier.size(300.dp)
        )

        Spacer(modifier = Modifier.height(16.dp))

        Text(
            text = "表情: ${avatarExpression ?: "待機中..."}",
            style = MaterialTheme.typography.bodyLarge
        )
    }
}

@Composable
fun AvatarDisplay(
    expression: String,
    modifier: Modifier = Modifier
) {

    Surface(
        modifier = modifier,
        color = MaterialTheme.colorScheme.surfaceVariant,
        shape = MaterialTheme.shapes.large
    ) {
        Box(
            contentAlignment = Alignment.Center,
            modifier = Modifier.fillMaxSize()
        ) {
            Text(
                text = when (expression) {
                    "laugh" -> "😄"
                    "sad" -> "😢"
                    "surprise" -> "😮"
                    "empathy" -> "🤗"
                    "confused" -> "😕"
                    else -> "😐"
                },
                style = MaterialTheme.typography.displayLarge
            )
        }
    }
}