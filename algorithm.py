import sys

def max_common_substring(str1, str2):
    max_len = 0
    max_substring = ""

    for i in range(len(str1)):
        for j in range(len(str2)):
            k = 0
            while (i + k < len(str1) and j + k < len(str2) and str1[i + k] == str2[j + k]):
                k += 1
            if k > max_len:
                max_len = k
                max_substring = str1[i:i + k]

    return max_substring

if len(sys.argv) != 3:
    print("Usage: python common_substring.py file1.txt file2.txt")
    sys.exit(1)

file1_path = sys.argv[1]
file2_path = sys.argv[2]

try:
    with open(file1_path, 'r') as file1, open(file2_path, 'r') as file2:
        content1 = file1.read()
        content2 = file2.read()

        common_substring = max_common_substring(content1, content2)
        print("Maximum common substring:", common_substring)

except FileNotFoundError:
    print("File not found.")
    sys.exit(1)
