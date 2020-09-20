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

Node* loop(Node* one) {
  Node* hare = one;
  Node* tortoise = one;

  while (1) {
	if (hare) {
	  hare = hare -> next;
	}
	if (hare) {
	  hare = hare -> next;
	}
	if (!hare) {
	  return NULL;
	}

	if (tortoise) {
	  if (tortoise -> next == hare) {
		break;
	  }
	  tortoise = tortoise -> next;
	}
  }

  return tortoise;
}

int main() {
  Node* A = (Node*) malloc(sizeof(Node));
  A -> value = 1;
  Node* B = (Node*) malloc(sizeof(Node));
  B -> value = 2;
  Node* C = (Node*) malloc(sizeof(Node));
  C -> value = 3;
  Node* D = (Node*) malloc(sizeof(Node));
  D -> value = 4;
  Node* E = (Node*) malloc(sizeof(Node));
  E -> value = 5;

  A -> next = B;
  B -> next = C;
  C -> next = D;
  D -> next = E;
  E -> next = C;
  std::cout << loop(A) -> value << std::endl;
  return 0;
}
