package com.example.echo_android.ui.feature.avatar

import androidx.compose.animation.core.FastOutSlowInEasing
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
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
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import com.example.echo_android.ui.feature.MainViewModel

@Composable
fun AvatarScreen(
    expression: String? = null,
    viewModel: MainViewModel = hiltViewModel()
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
    val infiniteTransition = rememberInfiniteTransition()
    val pulseScale by infiniteTransition.animateFloat(
        initialValue = 0.95f,
        targetValue = 1.05f,
        animationSpec = infiniteRepeatable(
            animation = tween(1000, easing = FastOutSlowInEasing),
            repeatMode = RepeatMode.Reverse
        ),
    )

    val backgroundColor = when (expression) {
        "laugh" -> Color(0xFFFFF9C4)
        "sad" -> Color(0xFFBBDEFB)
        "surprise" -> Color(0xFFFFCCBC)
        "empathy" -> Color(0xFFF8BBD0)
        "confused" -> Color(0xFFD7CCC8)
        else -> MaterialTheme.colorScheme.surfaceVariant
    }

    Surface(
        modifier = modifier,
        color = backgroundColor,
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
                    "empathy" -> "🥺"
                    "confused" -> "😕"
                    else -> "😐"
                },
                style = MaterialTheme.typography.displayLarge.copy(fontSize = 100.sp),
                modifier = Modifier.scale(pulseScale)
            )
        }
    }
}
