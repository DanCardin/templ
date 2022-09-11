from datetime import datetime
import re
from string import Formatter


class Git:
    def hash(self, *, long=False):
        if long:
            return 'askdfja'
        return 'asdfkjasdlfkjasdlfjsadlfj'


class Date:
    def today(self, *, format='iso'):
        today = datetime.today().date()
        if format == 'iso':
            return today.isoformat()
        return str(today)


class Context:
    git = Git()
    date = Date()


context = Context()


def bail(field_name):
    return f'{{{field_name}}}'


def parse(text):
    result = ''

    formatter = Formatter()
    for literal_text, field_name, _, _ in formatter.parse(text):
        result += literal_text

        if not field_name:
            continue

        namespace_name, method = field_name.split('.', 1)
        match = re.match(r'(\w+)(\(.*\))', method)
        if not match:
            result += bail(field_name)
            continue

        method_name, args = match.groups()

        literal_args = eval(f'dict{args}')

        namespace = getattr(context, namespace_name, None)
        if not namespace:
            result += bail(field_name)
            continue

        method = getattr(namespace, method_name, None)
        if not method:
            result += bail(field_name)
            continue

        try:
            method_result = method(**literal_args)
        except:
            result += bail(field_name)
            continue

        result += method_result
    return result

if __name__ == '__main__':
    text = "{date.today(format='iso')}-{git.hash(long=True)}-foo"
    result = parse(text)
    print(result)
