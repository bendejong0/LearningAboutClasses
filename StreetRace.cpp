#include <iostream>
using namespace std;

class Person{
private:
    bool speedticket(int mph){
        bool ticket = (mph > 60) ? true : false;
        if(ticket){
            return true;
        }
        else{
            return false;
        }
    }
public:
    int dist;
    int mph;
    int distanceCalc(int dist, int mph){
        double time = dist / mph;
        if(speedticket(mph)){
            cout << "You're getting a ticket!" << endl;
        }
        return time;
    }
};

int main(){
    cout << "Please enter the first person's distance: ";
    Person p1;
    cin >> p1.dist;
    cout << "Please enter the first person's mph: ";
    cin >> p1.mph;
    

    cout << "Please enter the second person's distance: ";
    Person p2;
    cin >> p2.dist;
    cout << "Please enter the second person's mph: "; 
    cin >> p2.mph;

    double person1 = p1.distanceCalc(p1.dist, p1.mph);
    double person2 = p2.distanceCalc(p2.dist, p2.mph);

    cout << "Person 1 finished in: " << person1 << " hours!" << endl;
    cout << "Person 2 finished in: " << person2 << " hours!"<< endl;

    return 0;
}
