# Noita Eye Glyphs Cipher Bruteforcer

Yes I'm trying to bruteforce the eye glyphs, you can check that off your bingo card.

The aim of this project is to find the cipher of (or at least one close to) the eye glyphs cipher by using a search base fuzzer. You can read more about fuzzing and search based fuzzing here : [https://www.fuzzingbook.org/html/SearchBasedFuzzer.html](https://www.fuzzingbook.org/html/SearchBasedFuzzer.html)

An earlier prototype in python can be found here [Cipher Bruteforcer](https://github.com/Azertinv/cipher_bruteforcer)

## How it works
1. Generate a random combination of ciphers to make a new cipher.
2. Encrypt a sample plaintext with this new cipher, this gives us a ciphertext.
3. Score this ciphertext against the measures from the eye glyphs.

For example: we see that the eye glyphs exhibit a uniform letter distribution (ignoring shared sections), therefore if the ciphertext we generated deviate from this behaviour, its score would be higher (lower score is better).

This score can be thought of as the measurements distance between the ciphertext and the eye glyphs, which should be correlated with the behavior distance of the generated cipher and the eye glyphs cipher.

## Cipher Decomposition
To generate a random cipher we need small and modular bricks. This is why we need to decompose known ciphers to hopefully extract those little nuggets of interesting behaviors.

### Example
Take an autokeyed-shift cipher which takes the last encrypted character as the key for the next character.

This cipher can be broken into 2 parts :
- the autokeyer which just take the last character encrypted and feed it to another cipher as a key
- the shift where the key is given by the autokeyer

In this case the shift cipher is a parameter for the autokeyer and can be swapped with another cipher pretty easily.

The autokeyer can also be replaced with a key generator based on the index.

## Cipher steps
- Shift
- Scramble
- Indexer
- Repeater
- Ciphertext Autokeyer

## Measurements
- Letter distribution
- Letter repeats
- Index bounds

## Contribute
The best way to contribute is by creating cipher steps and measurements. You can look at the issue list to find stuff to do.
If you can't code in rust yourself you can create an issue for a cipher step or a measure you feel is missing. You can also help decompositing ciphers and show your work by creating an issue, I'll implement your ideas.

### Adding a cipher checklist
- [ ] create a struct implementing Cipher
- [ ] scheduled mutation count is consistent with the mutate method
- [ ] gen func is added to ciphers::cipher_stack::random_cipher_step list
- [ ] if it can be an inner cipher impl InnerCipher for it, see Shift or Scramble
- [ ] tests for mutate and encrypt

### Adding a measure checklist
- [ ] create a struct implementing Measure
- [ ] measure is consistent whatever input data size we give it
- [ ] tests for measure and extraction

## Plans
A future issue I can see from this approach is if we have a bad testcase that's hard to mutate into a the perfect solution and another good but deadend textcase. What I mean is there is currently no way to avoid local minimum.
To fix that we need a way to measure the distance between 2 cipher. It's possible using a weighted Levenshtein distance but it's trying to fix a rich people problem, I want to focus on implementing a wide variety of cipher steps and measures first.

## How it REALLY works TODO
- cipher un/scheduled mutations
- queue/corpus scheduling
- distributions generation
- p value distance calculation
