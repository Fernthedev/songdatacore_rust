#pragma once

#include <future>
#include <thread>

#include "bindings.hpp"

namespace song_data_core
{

    std::promise<const BeatStarDataFile *> loadDatabaseAsync()
    {
        std::promise<const BeatStarDataFile *> promise;

        std::thread t([&]
                      { promise.set_value(Beatstar_RetrieveDatabase()); });
        t.detach();

        return promise;
    }

    template <typename K = void, typename V = void, typename Hasher = void>
    struct HashmapWrapper
    {
        const HashMap<K, V, Hasher> *hashmap;

        const std::function<size_t()> getLengthFunc;
        const std::function<K(size_t)> getKeyFunc;
        const std::function<V(K)> getValueFunc;
    };

#define CONVERT_RUST_TO_CPP_MAP(hashmap, funcPrefix, type) HashmapWrapper( \
    hashmap, [] { return funcPrefix##Len(hashmap); }, [](size_t index) { return funcPrefix##Get(hashmap, index); }, [](type key) { return funcPrefix##KeyGet(hashmap, key); })

    template <typename K = void>
    struct VectorWrapper
    {
        Vec<K> *vector;

        std::function<size_t()> getLengthFunc;
        std::function<K(size_t)> getValueFunc;
    };

#define CONVERT_RUST_TO_CPP_VEC(vec, funcPrefix) VectorWrapper( \
    vec, [] { return funcPrefix##Len(hashmap); }, [](size_t index) { return funcPrefix##Get(hashmap, index); })
}