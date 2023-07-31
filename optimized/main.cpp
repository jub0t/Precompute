#include <iostream>
#include <iomanip>
#include <locale>
#include <sstream>
#include <openssl/evp.h>
#include <deque>
#include <unordered_map>
#include <chrono>

using Combolist = std::deque<std::string>;

std::string sha256(const std::string &str)
{
    unsigned char hash[EVP_MAX_MD_SIZE];

    EVP_MD_CTX *mdctx;
    const EVP_MD *md = EVP_sha256();
    mdctx = EVP_MD_CTX_new();
    EVP_DigestInit_ex(mdctx, md, NULL);
    EVP_DigestUpdate(mdctx, str.c_str(), str.size());
    unsigned int hashLen;
    EVP_DigestFinal_ex(mdctx, hash, &hashLen);
    EVP_MD_CTX_free(mdctx);

    std::string result;
    result.reserve(2 * hashLen);
    for (unsigned int i = 0; i < hashLen; i++)
    {
        result.push_back("0123456789abcdef"[hash[i] >> 4]);
        result.push_back("0123456789abcdef"[hash[i] & 0x0F]);
    }

    return result;
}

Combolist combinations(std::string allowed)
{
    size_t charlen = allowed.length();
    Combolist list;

    if (charlen < 4)
    {
        return list;
    }

    for (size_t i = 0; i < charlen; i++)
    {
        for (size_t j = 0; j < charlen; j++)
        {
            for (size_t k = 0; k < charlen; k++)
            {
                for (size_t l = 0; l < charlen; l++)
                {
                    list.push_back(std::string(1, allowed[i]) + allowed[j] + allowed[k] + allowed[l]);
                }
            }
        }
    }

    return list;
}

int main()
{
    // Initialize
    std::string target = "278f14e96cc67489e5c0d6cebec8a2718fb158ec656fd41fed7ecd031cd472b2"; // "GOOD"
    std::unordered_map<std::string, std::string> hashes;

    // Initialize Combolist
    Combolist list = combinations("ABCDEFGHIJKLMNOPQRSTUVWXYabcdefghijklmnopqrstuvwxyz");
    std::cout << "COMBOLIST SIZE: " << list.size() << std::endl;
    std::size_t listSize = list.size();

    auto startTime = std::chrono::high_resolution_clock::now();
    // Hash The Entire  Combolist
    for (std::size_t i = 0; i < listSize; i++)
    {
        hashes[sha256(list[i])] = list[i];
    }

    auto endTime = std::chrono::high_resolution_clock::now();
    auto hashed_duration = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime).count();

    auto start = std::chrono::high_resolution_clock::now();
    std::unordered_map<std::string, std::string>::iterator found = hashes.find(target);
    if (found != hashes.end())
    {
        auto end = std::chrono::high_resolution_clock::now();
        auto found_duration = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();

        // Verbose
        std::cout << "HASH: " << found->first << "\nVALUE: " << found->second << "\nHASH TIME: " << hashed_duration << "ms"
                  << "\nFOUND TIME: " << found_duration << "ns"
                  << "\nHASHES/SEC: " << (listSize / (hashed_duration / 1000.0)) << std::endl;
    }
    else
    {
        std::cout << "Target Not Found." << std::endl;
    }

    return 0;
}