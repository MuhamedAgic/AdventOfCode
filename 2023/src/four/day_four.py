from copy import deepcopy


def day_4_part_1(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    card_scores = []
    for line in lines:
        card_nr = int(line.split(':')[0].split()[1])
        winning_nums = [int(x) for x in line.split('|')[0].split(':')[1].split()]
        my_nums =  [int(x) for x in line.split('|')[1].split()]

        print(f"Card {card_nr}\n    winning nums {winning_nums}\n    my nums {my_nums}")

        score_current_card = 0
        has_winning_num = False
        for my_num in my_nums:
            if my_num in winning_nums:
                if not has_winning_num: # als dit de eerste was +1
                    score_current_card += 1
                else:
                    score_current_card *= 2
                
                has_winning_num = True
            
            
        card_scores.append(score_current_card)
    
    ans = sum(card_scores)
    print("Answer part one: ", ans)
    return ans



def generate_next_level(scratchcards):
    # process eerste, maak nieuwe kaarten
    # process tweede, en alle kopiën van de tweede, zo laag voor laag.

    winning_nums_current_card_count = 0
    layer1 = deepcopy(scratchcards)

    all = [layer1]

    index_to_insert_copies = 0
    for idx, card in enumerate(scratchcards):
        print(f"Processing card {card[0][0]}\n    winning nums {card[0][1]}\n    my nums {card[0][2]}")
        for my_num in card[2]:
            if my_num in card[1]:
                winning_nums_current_card_count += 1  

        i = 0
        while i < winning_nums_current_card_count:
            if scratchcards[idx + i + 1][1] == True: # hij is al geprocessed
                continue

            card_to_copy = deepcopy(scratchcards[idx + i + 1]) # +1 want de volgende kaart kopieren
            print("Copying card:", idx + i + 1)
            #print("idx, i", idx, i)
            original_plus_new.insert(index_to_insert_copies + 1, card_to_copy) # processsed staat nog op false
            i += 1

        card[1] = True # processed = true

    for idx, item in enumerate(original_plus_new):
        if idx > 50:
            break
        print(f"NEW LIST {item[0]}\n    winning nums {item[1]}\n    my nums {item[2]}")

    return original_plus_new


def day_4_part_2(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()

    scratchcards = []
    scratch_cards_won = 0
    is_card_processed = False
    for idx, line in enumerate(lines):
        card_nr = int(line.split(':')[0].split()[1])
        winning_nums = [int(x) for x in line.split('|')[0].split(':')[1].split()]
        my_nums =  [int(x) for x in line.split('|')[1].split()]

        scratchcards.append([[card_nr, winning_nums, my_nums], is_card_processed])

        print(f"Card {card_nr}\n    winning nums {winning_nums}\n    my nums {my_nums}\n\n")
    
    original_plus_won_scratchcards = generate_more_scratchcards(scratchcards)

    ans = len(original_plus_won_scratchcards) # of winning_nums of my_nums
    print("Answer part two: ", ans)
    return ans


filename = "D:/git/magic/aoc2023/aoc2023/src/four/example_input.txt"
day_4_part_2(filename)