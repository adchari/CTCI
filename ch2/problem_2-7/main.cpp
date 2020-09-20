#include <algorithm>
#include <vector>
#include <map>
#include <queue>
#include <set>
#include <iostream>

using namespace std;

typedef struct node {
  int value;
  struct node* next;
} Node;

bool intersecting(Node* one, Node* two) {
  while (one -> next) {
	one = one -> next;
  }
  while (two -> next) {
	two = two -> next;
  }
  return one == two;
}

int main() {
  Node* first = (Node*) malloc(sizeof(Node));
  first -> value = 1;
  Node* first2 = (Node*) malloc(sizeof(Node));
  first2 -> value = 2;
  Node* second = (Node*) malloc(sizeof(Node));
  second -> value = 3;
  Node* third = (Node*) malloc(sizeof(Node));
  third -> value = 4;

  third -> next = NULL;
  second -> next = third;
  first -> next = second;
  first2 -> next = second;
  std::cout << intersecting(first, first2) << std::endl;
  return 0;
}
