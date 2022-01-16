if __name__ == '__main__':
    most_guesses_word = ""
    most_guesses = 0
    least_guesses_word = ""
    least_guesses = 100
    word_count = 0
    total_guesses = 0
    words_within_6 = 0
    for line in open('test_results/result_smaller_words.txt', 'r').readlines():
        try:
            word, num = line.split()
            word = word.strip('"')
            num = int(num)
        except:
            continue

        word_count += 1
        total_guesses += num

        if num <= 6:
            words_within_6 += 1

        if num < least_guesses:
            least_guesses = num
            least_guesses_word = word

        if num > most_guesses:
            most_guesses = num
            most_guesses_word = word

    print(f'Average num guesses: {total_guesses / word_count}')
    print(f'ratio that win within 6: {words_within_6 / word_count}')
    print(f'Worst: {most_guesses_word} - {most_guesses}')
    print(f'Best: {least_guesses_word} - {least_guesses}')