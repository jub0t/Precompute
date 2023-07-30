#include <iostream>
#include <iomanip>
#include <sstream>
#include <openssl/evp.h>
#include <deque>
#include <unordered_map>
#include <chrono>

using Combolist = std::deque<std::string>;

std::string sha256(const std::string &str)
{
    unsigned char hash[EVP_MAX_MD_SIZE]; // Maximum size for any hash algorithm

    EVP_MD_CTX *mdctx;
    const EVP_MD *md = EVP_sha256(); // Get the SHA-256 message digest algorithm
    mdctx = EVP_MD_CTX_new();
    EVP_DigestInit_ex(mdctx, md, NULL);
    EVP_DigestUpdate(mdctx, str.c_str(), str.size());
    unsigned int hashLen;
    EVP_DigestFinal_ex(mdctx, hash, &hashLen);
    EVP_MD_CTX_free(mdctx);

    // Convert hash to hexadecimal string representation
    std::stringstream result;
    result << std::hex << std::setfill('0');
    for (unsigned int i = 0; i < hashLen; i++)
    {
        result << std::setw(2) << static_cast<unsigned int>(hash[i]);
    }

    return result.str();
}

Combolist combinations(std::string allowed)
{
    Combolist list;
    size_t charlen = allowed.length();

    if (charlen < 4)
    {
        // Edge case: If the length of the allowed string is less than 4,
        // it is not possible to form 4-letter combinations.
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
    std::string target = "278f14e96cc67489e5c0d6cebec8a2718fb158ec656fd41fed7ecd031cd472b2"; // "GOOD"

    Combolist list = combinations("ABCDEFGHIJKLMNOPQRSTUVWXYabcdefghijklmnopqrstuvwxyz1234567890"); // Include 'Y' and fix 'Z'
    std::cout << "Combolist Size: " << list.size() << std::endl;                                    // Use cout instead of printf
    std::unordered_map<std::string, std::string> hashes;

    auto startTime = std::chrono::high_resolution_clock::now();

    for (const std::string &word : list)
    {
        std::string hash = sha256(word);
        hashes[hash] = word;
    }

    auto endTime = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime).count();
    std::cout << "Hashed in " << duration << "ms" << std::endl;

    auto start = std::chrono::high_resolution_clock::now();
    auto found = hashes.find(target);

    if (found != hashes.end())
    {
        auto end = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::nanoseconds>(end - start).count();
        std::cout << "Target Found: " << found->first << " in " << duration << "ns" << std::endl;
    }
    else
    {
        std::cout << "Target Not Found." << std::endl;
    }

    return 0;
}
