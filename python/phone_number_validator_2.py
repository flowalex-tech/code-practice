#!/usr/bin/env python3

""" Given a text file file.txt that contains list of phone numbers (one per line), write a one liner bash script to print all valid phone numbers.
You may assume that a valid phone number must appear in one of the following two formats: (xxx) xxx-xxxx or xxx-xxx-xxxx. (x means a digit)
 You may also assume each line in the text file must not contain leading or trailing white spaces.

Example:

Assume that file.txt has the following content:

987-123-4567
123 456 7890
(123) 456-7890
Your script should output the following valid phone numbers:

987-123-4567
(123) 456-7890
From https://leetcode.com/problems/valid-phone-numbers/
This version will actually test that it is a valid phone number on an exchange
"""
import phonenumbers
import re

phone_file = open("file.txt", "r")

for i in phone_file:
    phone = phonenumbers.parse(i, "US")
    if phonenumbers.is_valid_number(phone):
        print(i)
            
phone_file.close
