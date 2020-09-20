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
  };

private:
  node* head;
  int size;
  
public:
  Stack() {
	head = NULL;
	size = 0;
  }
  
  void push(int val);
  int pop();
  int getSize();
};

void Stack::push(int val) {
  node* n = new node;
  n -> val = val;
  n -> next = this -> head;
  this -> head = n;
  this -> size++;
}

int Stack::pop() {
  if (!this -> head) {
	return -1;
  }

  node* h = this -> head;
  this -> head = this -> head -> next;
  this -> size--;
  return h -> val;
}

int Stack::getSize() {
  return this -> size;
}


class StackStack {
private:
  int cap;
  vector<Stack> stacks;
  
public:
  StackStack() {
	cap = 10;
  }

  StackStack(int c) {
	cap = c;
  }

  void push(int);
  int pop();
};

void StackStack::push(int v) {
  if (this -> stacks.size() == 0 || this -> stacks[stacks.size() - 1].getSize() == cap) {
	Stack x;
	this -> stacks.push_back(x);
  }

  this -> stacks[stacks.size() - 1].push(v);
}

int StackStack::pop() {
  if (this -> stacks.size() == 0) {
	return -1;
  }

  int ret = this -> stacks[stacks.size() - 1].pop();
  if (this -> stacks[stacks.size() - 1].getSize() == 0) {
	this -> stacks.pop_back();
  }
  return ret;
}


int main() {
  return 0;
}

