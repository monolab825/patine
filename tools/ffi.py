from lxml import etree
import re

def main():
    file = open("./target/yul.html", 'r', encoding='utf-8')
    content = file.read()

    tree = etree.HTML(content)

    # xpath = '/html/body/div/div[2]/div/div/div[2]/div/section/section[5]/section[4]/div[1]/table/tbody/tr'
    xpath = "//*[@class='row-even'] | //*[@class='row-odd']"

    elements = tree.xpath(xpath)

    for elem in elements:
        comment = elem[3][0].text.replace("\n", " ")
        print("/// %s"%(comment,))

        func = elem[0][0].text.replace("\n", "").replace(" ", "")
        splited = re.split('[(,)]', func)
        splited = [item for item in splited if item.strip()]


        target = "pub fn __yul_" + splited[0] + '('

        for i in range(1, len(splited)):
            target += splited[i] + ": NativeType, "

        target += ') -> NativeType;'

        print(target)





main()
