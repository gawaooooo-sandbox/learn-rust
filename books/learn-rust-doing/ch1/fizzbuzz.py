# PythonでFizzBuzz問題を解く
# 1から100まで繰り返す
for i in range(1, 101):
    # 条件を一つずつ判定する
    if i % 3 == 0 and i % 5 == 0:
        # 3でも5でも割り切れる場合はFizzBuzz
        print('FizzBuzz')
    elif i % 3 == 0:
        # 3で割り切れる場合はFizz
        print('Fizz')
    elif i % 5 == 0:
        # 5で割り切れる場合はBuzz
        print('Buzz')
    else:
        # それ以外は数値をそのまま出力する
        print(i)
