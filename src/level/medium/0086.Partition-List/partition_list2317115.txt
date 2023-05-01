// https://leetcode.com/problems/partition-list/solutions/2317115/c-rust-accepted-two-pointer/
class Solution {
public:
	ListNode* partition(ListNode* head, int x) {      
		ListNode* h1 = new ListNode(0);        
		ListNode* h2 = new ListNode(0);
		ListNode *p1 = h1, *p2=h2;
		while(head){
			if(head->val < x){
				p1->next = head;
				head = head->next;
				p1 = p1->next;
				p1->next = NULL;
			}else{
				p2->next = head;
				head = head->next;
				p2 = p2->next;
				p2->next = NULL; 
			}
		}
		p1->next = h2->next;
		return h1->next;
	}
};