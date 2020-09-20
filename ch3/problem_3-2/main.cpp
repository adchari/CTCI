#include <algorithm>
#include <vector>
#include <map>
#include <queue>
#include <set>
#include <iostream>

using namespace std;

class Stack {
  struct node {
	int val;
	node* next;
	node* nextMin;
  };

private:
  node* head;
  node* min;
  
public:
  Stack() {
	head = NULL;
	min = NULL;
  }
  
  void push(int val);
  int pop();
  int minVal();
};

void Stack::push(int val) {
  node* n = new node;
  n -> val = val;
  n -> next = this -> head;
  this -> head = n;

  if (!(this -> min)) {
	this -> min = n;
	n -> nextMin = NULL;
  } else if (this -> min -> val > val) {
	n -> nextMin = this -> min;
	this -> min = n;
  } else {
	n -> nextMin = NULL;
  }
}

int Stack::pop() {
  if (!this -> head) {
	return -1;
  }

  node* h = this -> head;
  if (h == this -> min) {
	this -> min = h -> nextMin;
  }
  this -> head = this -> head -> next;
  return h -> val;
}

int Stack::minVal() {
  if (!this -> min) {
	return -1;
  }
  return this -> min -> val;
}

int main() {
  Stack s;
  s.push(3);
  std::cout << s.minVal() << std::endl;
  s.push(2);
  s.push(4);
  std::cout << s.minVal() << std::endl;
  s.push(1);
  std::cout << s.minVal() << std::endl;
  s.pop();
  std::cout << s.minVal() << std::endl;
  s.pop();
  s.pop();
  std::cout << s.minVal() << std::endl;
  
  return 0;
}

