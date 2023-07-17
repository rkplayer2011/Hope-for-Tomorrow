public class HopeForTomorrow {
	
	//Variable declaration
	int happiness = 0;
	int hope = 0;
	
	//Constructor
	public HopeForTomorrow() {
		this.happiness = 0;
		this.hope = 0;
	}
	
	//Method to increase happiness
	public void increaseHappiness() {
		this.happiness++;
		this.hope++;
	}
	
	//Method to decrease happiness
	public void decreaseHappiness() {
		
		if (this.happiness > 0) {
			this.happiness--;
			this.hope--;
		}
		
	}
	
	//Method to set happiness
	public void setHappiness(int newHappiness) {
		this.happiness = newHappiness;
		this.hope = newHappiness;
	}
	
	//Method to get happiness
	public int getHappiness() {
		return this.happiness;
	}
	
	//Method to get hope
	public int getHope() {
		return this.hope;
	}
	
	//Method to increase hope
	public void increaseHope() {
		this.hope++;
	}
	
	//Method to decrease hope
	public void decreaseHope() {
		
		if (this.hope > 0) {
			this.hope--;
		}
		
	}
	
	//Method to test if hope is greater than happiness
	public boolean hopeIsGreaterThanHappiness() {
		
		if (this.hope > this.happiness) {
			return true;
		}
		else {
			return false;
		}
		
	}
	
	//Main method
	public static void main (String[] args) {
		
		//Create an instance of the HopeForTomorrow class
		HopeForTomorrow hft = new HopeForTomorrow();
		
		//Increase happiness 3 times
		for (int i = 0; i < 3; i++) {
			hft.increaseHappiness();
		}
		
		//Set happiness to 10
		hft.setHappiness(10);
		
		//Decrease happiness 1 time
		hft.decreaseHappiness();
		
		//Increase hope 5 times
		for (int i = 0; i < 5; i++) {
			hft.increaseHope();
		}
		
		//Display happiness and hope
		System.out.println("Happiness: " + hft.getHappiness());
		System.out.println("Hope: " + hft.getHope());
		
		//Test if hope is greater than happiness
		if (hft.hopeIsGreaterThanHappiness()) {
			System.out.println("Hope is greater than happiness!");
		}
		else {
			System.out.println("Happiness is greater than hope!");
		}
		
	}

}