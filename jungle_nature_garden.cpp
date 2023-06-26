#include <iostream> 
using namespace std; 
  
// Function to print the string 
void printString(string s) 
{ 
    cout << s; 
} 
  
// Driver code 
int main() 
{ 
    string st = "Hope for Tomorrow"; 
  
    printString(st); 
      
    for (int i=0; i<2000; i++){
		cout << "Hope for Tomorrow" << endl;
	}
    return 0; 

}