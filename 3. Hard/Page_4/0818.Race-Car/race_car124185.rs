// https://leetcode.com/problems/race-car/solutions/124185/c-3ms-solution-o-logt-time-and-space/
#include <limits.h>

//#define SHOWDEBUG
//#define SHOWDEBUG2

class Solution 
{
    struct CarStatus
    {
        #ifdef SHOWDEBUG
        int id;
        #endif
        int balance;
        int commandCount;
        int target;
    };
public:
    int racecar(int target)
    {
        if( target == 0 )
            return 0;

        //populate a lookup array so that index = number of A commands, and value = distance travelled
        int values[32];
        for( int index = 0; index < 32; index++ )
            values[index] = INT_MAX >> (31 - index);
        
        //Create the initial deque entry
        deque<CarStatus> statuses;
        CarStatus carStatus;
        carStatus.balance = 0;
        carStatus.commandCount = 0;
        carStatus.target = target;
        #ifdef SHOWDEBUG
        int id = 0; // for tracking parent/child connections while debugging
        carStatus.id = id++;
        #endif
        statuses.push_back(carStatus);
        
        //Go through the deque
        int bestCount = INT_MAX; // variable to hold the answer answer in
        deque<CarStatus>::iterator parentCarStatus = statuses.begin();        
        while( parentCarStatus != statuses.end() )
        {
            // queue is in ascending order by commandCount, so stop when we reach the bestCount value
            if( parentCarStatus->commandCount >= bestCount - 1 )
            {
                #ifdef SHOWDEBUG2
                cout << "stopping cc " << parentCarStatus->commandCount << " best " << bestCount << endl;
                #endif
                break;
            }
            for( int index = 1; index < 32 && target != 0; index++ )
            {
                int currentValue = values[index];
                int absTarget = parentCarStatus->target < 0 ? 0 - parentCarStatus->target : parentCarStatus->target;
                if( currentValue < (absTarget >> 1) )
                {
                    #ifdef SHOWDEBUG2
                    cout << "skipping absTarget " << absTarget << " value " << currentValue << endl;
                    #endif
                    continue;
                }
                if( currentValue > (absTarget << 1) )
                {
                    #ifdef SHOWDEBUG2
                    cout << "stopping absTarget " << absTarget << " value " << currentValue << endl;
                    #endif
                    break;
                }
                carStatus = *parentCarStatus;
                carStatus.commandCount += index + 1;
                if( carStatus.commandCount > bestCount )
                {
                    #ifdef SHOWDEBUG2
                    cout << "stopping cc " << carStatus.commandCount << " best " << bestCount << endl;
                    #endif
                    break;
                }
                if( carStatus.target > 0 )
                {
                    carStatus.balance++;
                    carStatus.target -= currentValue;
                }
                else
                {
                    carStatus.balance--;
                    carStatus.target += currentValue;
                }
                #ifdef SHOWDEBUG
                carStatus.id = id++;
                cout << "cc " << carStatus.commandCount 
                    << "  balance " << carStatus.balance
                    << "  pid " << parentCarStatus->id
                    << "  id " << carStatus.id
                    << "  dsize " << statuses.size()
                    << "  target " << carStatus.target
                    << "  value " << (parentCarStatus->target > 0 ? currentValue : 0 - currentValue)
                    << "  bestCount " << bestCount
                    << endl;
                #endif
                if( carStatus.target == 0 )
                {
                    // adjust commandCount for balance;
                    if( carStatus.balance == 0 )
                        carStatus.commandCount--;
                    else if( carStatus.balance > 0 )
                        carStatus.commandCount += carStatus.balance - 2;
                    else // if( carStatus.balance < 0 )
                        carStatus.commandCount -= carStatus.balance + 1;
                    // check if adjusted commandCount is better than bestCount
                    if( carStatus.commandCount < bestCount )
                        bestCount = carStatus.commandCount;
                    break;
                }
                // insert child in deque ordered by commandCount
                deque<CarStatus>::iterator insertItr = next( parentCarStatus );
                while( insertItr != statuses.end() && carStatus.commandCount >= insertItr->commandCount )
                    insertItr++;
                statuses.insert( insertItr, carStatus );
            }
            #ifdef SHOWDEBUG2
            cout << "size " << statuses.size() << endl;
            #endif
            parentCarStatus++;
        }
            
        return bestCount;
    }
};