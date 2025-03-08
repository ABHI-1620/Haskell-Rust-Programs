power :: Int -> Int -> Int
power _ 0 = 1       -- Any number raised to the power of 0 is 1
power base exp = base * power base (exp - 1)