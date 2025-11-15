package com.example.echo_android.ui.feature.auth

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.LocalContentColor
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch

@Composable
fun AuthScreen(
    title: String,
    buttonText: String,
    onAuthClick: suspend (username: String, password: String) -> Boolean,
    onSecondaryClick: () -> Unit,
    secondaryText: String
    ) {
    // todo: usernameとpasswordが空白の場合などエラーを受け取ったらエラーメッセージを表示する
    var username by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var loading by remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()

    Column {
        Text(text = title)

        TextField(
            value = username,
            onValueChange = {username = it},
            label = {Text("username")},
            singleLine = true,
            maxLines = 1
        )

        TextField(
            value = password,
            onValueChange = {password = it},
            label = {Text("password")},
            singleLine = true,
            maxLines = 1
        )

        Button(
            enabled = !loading,
            onClick = {
                loading = true
                scope.launch {
                    val success = onAuthClick(username, password)
                    loading = false
                    if (success) {
                        Log.d("AuthScreen", "$title successful")
                    }
                }
            },
        ) {
            if (loading) {
                Loading()
            } else {
                Text(text = buttonText)
            }
        }

        TextButton(
            onClick = onSecondaryClick
        ) {
            Text(text = secondaryText)
        }
    }
}


@Composable
private fun Loading() {
    CircularProgressIndicator(
        modifier = Modifier.size(24.dp),
        color = LocalContentColor.current,
        strokeWidth = 2.dp,
    )
}