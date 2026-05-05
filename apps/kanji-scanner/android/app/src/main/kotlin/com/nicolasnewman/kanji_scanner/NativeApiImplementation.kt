package com.nicolasnewman.kanji_scanner

import FlutterError
import GetCardsForModel
import GetDecks
import GetModelFields
import GetModels
import GetNotesWithFieldsForModel
import GetModelsWithInfo
import NativeApi
import kotlinx.coroutines.asCoroutineDispatcher
import kotlinx.coroutines.runBlocking
import java.util.concurrent.Executors

class NativeApiImplementation(val mAnkiDroid: AnkiDroidHelper) : NativeApi {
    private val backgroundDispatcher = Executors.newSingleThreadExecutor().asCoroutineDispatcher()

    override fun getDecks(): GetDecks {
        if (!mAnkiDroid.hasPermission()) {
            throw FlutterError(
                "PERMISSION_DENIED",
                "AnkiDroid permission not granted",
                null
            )
        }
        val decks = mAnkiDroid.getDecks() ?: emptyMap()
        return GetDecks(decks)
    }

    override fun getModels(): GetModels {
        if (!mAnkiDroid.hasPermission()) {
            throw FlutterError(
                "PERMISSION_DENIED",
                "AnkiDroid permission not granted",
                null
            )
        }
        val decks = mAnkiDroid.getModels() ?: emptyMap()
        return GetModels(decks)
    }

    override fun getModelsWithInfo(): GetModelsWithInfo {
        if (!mAnkiDroid.hasPermission()) {
            throw FlutterError(
                "PERMISSION_DENIED",
                "AnkiDroid permission not granted",
                null
            )
        }
        val models = mAnkiDroid.getModelsWithInfo()
        return GetModelsWithInfo(models)
    }

    override fun getModelFields(modelId: Long): GetModelFields {
        if (!mAnkiDroid.hasPermission()) {
            throw FlutterError(
                "PERMISSION_DENIED",
                "AnkiDroid permission not granted",
                null
            )
        }
        val fields = mAnkiDroid.getModelFieldNames(modelId)
        return GetModelFields(fields)
    }

    override fun getNotesWithFieldsForModel(modelId: Long, fieldName: String): GetNotesWithFieldsForModel {
        if (!mAnkiDroid.hasPermission()) {
            throw FlutterError(
                "PERMISSION_DENIED",
                "AnkiDroid permission not granted",
                null
            )
        }
        return runBlocking(backgroundDispatcher) {
            val notes = mAnkiDroid.getNotesWithFieldsForModel(modelId, fieldName)
            GetNotesWithFieldsForModel(notes)
        }
    }

    override fun getCardsForModel(modelId: Long, fieldName: String, offset: Long, limit: Long): GetCardsForModel {
        if (!mAnkiDroid.hasPermission()) {
            throw FlutterError(
                "PERMISSION_DENIED",
                "AnkiDroid permission not granted",
                null
            )
        }

        return runBlocking(backgroundDispatcher) {
            val cards = mAnkiDroid.getCardsForModel(modelId, fieldName, offset, limit)
            GetCardsForModel(cards)
        }
    }
}