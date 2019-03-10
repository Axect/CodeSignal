def reverseInParentheses(inputString):
    s = inputString
    for i in range(len(s)):
        if s[i] == "(":
            start = i
        if s[i] == ")":
            end = i
            return reverseInParentheses(s[:start] + s[start+1:end][::-1] + s[end+1:])
    return s

test = [str(i) for i in range(1, 1000000)]
test.append("(123)321(456)(789)(123)(321)321(123)(321)")

print(reverseInParentheses("".join(test)))
