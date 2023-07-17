-- Hope for Tomorrow

--message.lua

--Function to print a message
function printMsg(message)
    print(message)
end

--Function to print a greeting
function printGreeting()
    printMsg("Welcome to Hope for Tomorrow")
end

--Print the greeting
printGreeting()

--Function to print information
function printInfo()
    printMsg("We are here to help you and bring hope for tomorrow.")
end

--Print the info
printInfo()

--Function to print encouragement
function printEncouragement()
    printMsg("You can do it, never give up on your dreams.")
end

--Print the encouragement
printEncouragement()

--Function to print a goodbye
function printGoodbye()
    printMsg("Thank you for visiting Hope for Tomorrow, goodbye.")
end

--Print the goodbye
printGoodbye()

--Variable to store the website address
websiteAddress = "hft.example.com"

--Function to print the website address
function printWebsiteAddress()
    printMsg("Visit us online at " .. websiteAddress)
end

--Print the website address
printWebsiteAddress()

--Function to print a donation request
function printDonationRequest()
    printMsg("Please consider donating to Hope for Tomorrow.")
end

--Print the donation request
printDonationRequest()

--Function to print a closing message
function printClosingMsg()
    printMsg("Our work is for a brighter tomorrow.")
end

--Print the closing message
printClosingMsg()

--Function to print a contact message
function printContactMsg()
    printMsg("If you'd like to contact us, please visit our website.")
end

--Print the contact message
printContactMsg()

--Function to open the website
function openWebsite()
    system.openUrl(websiteAddress)
end

--Function to print a link to the website
function printWebsiteLink()
    printMsg("Click here to visit " .. websiteAddress)
end

--Function to print instructions for opening the website
function printWebsiteInstructions()
    printMsg("To open our website, click the link above or copy and paste the address into your browser.")
end

--Function to print a message asking for donations
function printDonationMsg()
    printMsg("If you'd like to help us, please consider donating to Hope for Tomorrow.")
end

--Print the donation message
printDonationMsg()

--Function to print a message about volunteering
function printVolunteerMsg()
    printMsg("We are always looking for volunteers. If you'd like to help, please contact us.")
end

--Print the volunteer message
printVolunteerMsg()

--Function to print a message of hope
function printHope()
    printMsg("Together, we can bring hope for tomorrow and a brighter future.")
end

--Print the message of hope
printHope()