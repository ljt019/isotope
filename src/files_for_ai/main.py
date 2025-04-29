import time

class Room:
    def __init__(self, description, exits, items):
        self.description = description
        self.exits = exits
        self.items = items

class Game:
    def __init__(self):
        self.rooms = {
            'start': Room(
                'You are in a dimly lit room. There\'s a key on the table.',
                {'east': 'corridor'},
                ['key']
            ),
            'corridor': Room(
                'A long corridor with a locked door to the east.',
                {'west': 'start', 'east': 'corridor'},
                []
            ),
            'control_room': Room(
                'A room with a terminal. There\'s a password prompt.',
                {'west': 'corridor'},
                ['terminal']
            ),
            'exit': Room('You escaped! Congratulations!', {}, [])
        }
        self.current_room = 'start'
        self.inventory = []
        self.terminal_unlocked = False

    def display_room(self):
        print('\n' + self.rooms[self.current_room].description)
        if self.rooms[self.current_room].items:
            print('Items here:', ', '.join(self.rooms[self.current_room].items))

    def handle_input(self, command):
        room = self.rooms[self.current_room]
        
        if command == 'quit':
            print('Thanks for playing!')
            return False
        
        if command == 'inventory':
            print('Inventory:', ', '.join(self.inventory) if self.inventory else 'Nothing')
            return True
        
        if 'key' in room.items and command == 'take key':
            self.inventory.append('key')
            room.items.remove('key')
            print('You picked up the key.')
            return True
        
        if self.current_room == 'corridor' and 'key' in self.inventory and command == 'use key':
            print('You unlock the eastern door.')
            self.rooms['corridor'].exits['east'] = 'exit'
            return True
        
        if self.current_room == 'control_room' and 'terminal' in room.items and not self.terminal_unlocked:
            password = input('Enter password: ')
            if password == '1234':
                print('Terminal unlocked!')
                self.terminal_unlocked = True
                self.rooms['corridor'].exits['east'] = 'exit'
            else:
                print('Wrong password!')
            return True
        
        if command in room.exits:
            self.current_room = room.exits[command]
            return True
        
        print("Invalid command. Try: west, east, take key, use key, inventory, or quit")
        return True

    def play(self):
        print('Welcome to the Adventure Game!')
        time.sleep(1)
        
        while True:
            self.display_room()
            command = input('\n> ').lower()
            if not self.handle_input(command):
                break
        
if __name__ == '__main__':
    game = Game()
    game.play()