package com.example.echo_android.ui

import android.R.attr.horizontalDivider
import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.toMutableStateList
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import com.example.echo_android.apolloClient
import com.example.rocketreserver.GetTimelineQuery

@Composable
fun TimelineScreen() {
    // sample data
    val posts = remember {
        (0..20).map { i ->
            Post(
                id = i,
                username = "user$i",
                text = "Hello, World!",
                reactions = emptyMap()
            )
        }.toMutableStateList()
    }

    LaunchedEffect(Unit) {
        Log.d("TimelineScreen", "Starting query...")

        val response = apolloClient.query(GetTimelineQuery()).execute()
        Log.d("TimelineScreen", "Response received")
        Log.d("TimelineScreen", "Has errors: ${response.hasErrors()}")
        Log.d("TimelineScreen", "Errors: ${response.errors}")
        Log.d("TimelineScreen", "Data: ${response.data}")
        Log.d("TimelineScreen", "Exception: ${response.exception}")
    }

    LazyColumn(
        modifier = Modifier.fillMaxSize()
    ) {
        items(posts) { post ->
            PostItem(
                post = post
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
fun TimelineScreenPreview() {
    MaterialTheme {
        TimelineScreen()
    }
}


