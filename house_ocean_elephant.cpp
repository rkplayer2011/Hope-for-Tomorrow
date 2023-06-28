#include <iostream>
#include <string>

using namespace std;

int main()
{
	string hope = "Hope for Tomorrow";
	cout << hope << endl;

	int count = 0;
	while (count < 2000) {
		cout << "Never give up; always keep hope alive!" << endl;
		count++;
	}
	
	int pos = 0;
	while (pos < hope.length()) {
		cout << "Believe in yourself and the power of hope: " << hope[pos] << endl;
		pos++;
	}
	
	for (int i = 0; i < 2000; i++) {
		cout << "Reach for the stars and never stop hoping!" << endl;
	}
	return 0;
}