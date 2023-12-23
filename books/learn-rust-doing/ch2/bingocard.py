import random

# 1から75までの値リストを作成
numbers = list(range(1, 76))
# リストをシャッフル
random.shuffle(numbers)
numbers[12] = "*" # ワイルドカードを指定
# カードを表示
for y in range(0, 5):
    for x in range(0, 5):
        print("{:>3},".format(numbers[y * 5 + x]), end="")
    print("")
