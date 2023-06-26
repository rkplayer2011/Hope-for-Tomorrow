<?php 

//defining the class
class HopeForTomorrow
 {
 	//declaring the properties
 	public static $title;
 	public static $author;

 	//constructor
 	public function __construct($title,$author)
 	{
 		self::$title = $title;
 		self::$author = $author;
 	}

 	public static function getInfo()
 	{
 		$info = 'The book title is ' . self::$title . ' and the author is ' . self::$author . '.';
 		return $info;
 	}

 }

 //instantiating the class
 $book = new HopeForTomorrow('Hope for Tomorrow', 'John Doe');

 //calling the method
 echo HopeForTomorrow::getInfo(); 

//defining the constants
define('DISCOUNT',10);
define('SHIPPING_CHARGE', 20);

//declaring the variable
$price = 50;

//calculating the discount
$discount = $price * (DISCOUNT / 100);

//calculating the total price
$total = $price - $discount + SHIPPING_CHARGE;

//printing the total price
echo 'The total price is ' . $total . '.';

//adding the book to cart
function addToCart($book)
{
	$cart = array();
	array_push($cart, $book);
	echo 'The book is added to your cart.';
}

//calling the function
addToCart($book);
 
//defining the function to get reviews
function getReviews()
{
	$reviews = array('Good book', 'Very inspiring', 'Must read!');
	return $reviews;
}

//calling the function 
$reviews = getReviews();

//printing the reviews
 foreach($reviews as $review) 
 {
 	echo $review . '<br>';
 }
 
 //defining the function to increase sales
 function increaseSales() 
 {
 	$sales = 100;
 	echo 'The current sales are ' . $sales;
 	$sales = $sales + 20;
 	echo 'The updated sales are ' . $sales;
 }
 
 //calling the function
 increaseSales();
 
 //defining the function to get ratings
 function getRatings($rating)
 {
	if($rating >= 4.5) 
	{
		echo 'Excellent book!';
	} 
	elseif($rating >= 4.0) 
	{
		echo 'Great book!';
	} 
	elseif($rating >= 3.5) 
	{
		echo 'Good book!';
	} 
	elseif($rating >= 3.0) 
	{
		echo 'Average book!';
	} 
	else
	{
		echo 'Poor book!';
	}
}

//calling the function
getRatings(4.8);

//defining the function to send messages
function sendMessage($recipient, $message)
{
	$recipient_name = $recipient;
	$message_text = $message;
	echo 'Message sent to ' . $recipient_name . '.';
}

//calling the function
sendMessage('John Doe', 'Hope you enjoy the book!');
 
//defining the function to update book details
function updateBook($book) 
{
	$book->title = 'Hope for a Better Tomorrow';
	$book->author = 'John Smith';
	echo 'Book details updated successfully!';
}

//calling the function
updateBook($book);

//defining the function to display books
function displayBooks() 
{
	$book_list = array('Hope for Tomorrow', 'A New Beginning', 'The Power of Positive Thinking');
	foreach($book_list as $book) 
	{
		echo $book . '<br>';
	}
}

//calling the function
displayBooks();

//defining the function to get book details from database
function getBookDetails()
{
	$host = 'localhost';
	$user = 'root';
	$password = '';
	$dbname = 'book_details';

	// creating the connection
	$conn = new mysqli($host, $user, $password, $dbname);

	if($conn->connect_error)
	{
		die('Connection failed: ' . $conn->connect_error);
	}

	// executing the query
	$sql = 'SELECT * FROM books;';
	$result = $conn->query($sql);

	if($result->num_rows > 0)
	{
		// fetching the records
		while($row = $result->fetch_assoc())
		{
			echo 'Title: ' . $row['title'] . '<br>';
			echo 'Author: ' . $row['author'] . '<br>';
			echo '<br>';
		}
	} 
	else 
	{
		echo 'No books found';
	}

	// closing the connection
	$conn->close();

}

//calling the function
getBookDetails();

//defining the function to get book recommendations
function getRecommendations() 
{
	$books_array = array('Hope for Tomorrow', 'The Power of Positive Thinking', 'A New Beginning');
	$book_recommendations = array_rand($books_array);
	echo 'We recommend ' . $books_array[$book_recommendations] . '.';
}

//calling the function
getRecommendations();

?>