firstTwoElements :: [a] -> [a]
firstTwoElements (x:y:_) = [x, y]
firstTwoElements xs = xs