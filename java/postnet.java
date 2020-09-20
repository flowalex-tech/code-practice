import java.io.*; 
import java.util.Scanner;
import javax.swing.JFileChooser;
public class postnet {
    public static void main(String[] args) throws FileNotFoundException {
    // Lets the user select the file
        JFileChooser chooser = new JFileChooser();
        chooser.showOpenDialog(null);
        File fileObj = chooser.getSelectedFile();
            PrintWriter addressout = new PrintWriter ("labels.txt");
            // Separates the text file into different fields
        Scanner in;
        try {
            in = new Scanner(fileObj);
            // Reading multiple lines and convert to a string
            while (in.hasNextLine()){
                String line = in.nextLine();
                String[] inaddress = line.split(",");
                String name = inaddress[0];
                String address = inaddress[1];
                String city = inaddress[2];
                String state = inaddress[3];
                String zip = inaddress[4];
                //Takes the information from the file and organizes as a mailing address and adds the postnet barcode
                //File only prints the last entry, but the console outputs all of them
                addressout.println(name +"\n" + address +"\n" + city + " " + state + " " + zip + "\n" + getBarCode(zip) + "\n");
                //Tests output and makes sure it matches the file
                System.out.print(name +"\n" + address +"\n" + city + " " + state + " " + zip + "\n" + getBarCode(zip) + "\n");
            }
            addressout.close();
        }
        catch (FileNotFoundException e) {
            e.printStackTrace();
            System.out.println("ERROR: File Not Found");
        }
    }
    public static String getBarCode(String zip) {
        Scanner in = new Scanner(zip);
        String line = in.nextLine();
        String[] fields = line.split("-");
        String zipCodeReformat = (fields[0] + fields[1]);
        char num0 = zipCodeReformat.charAt(0);
        char num1 = zipCodeReformat.charAt(1);
        char num2 = zipCodeReformat.charAt(2);
        char num3 = zipCodeReformat.charAt(3);
        char num4 = zipCodeReformat.charAt(4);
        char num5 = zipCodeReformat.charAt(5);
        char num6 = zipCodeReformat.charAt(6);
        char num7 = zipCodeReformat.charAt(7);
        char num8 = zipCodeReformat.charAt(8);
        // add the sum of the numbers for the check sum
        int zipSum = Character.getNumericValue(num0)+Character.getNumericValue(num1)+Character.getNumericValue(num2)+Character.getNumericValue(num3)+Character.getNumericValue(num4)+Character.getNumericValue(num5)+Character.getNumericValue(num6)+Character.getNumericValue(num7)+Character.getNumericValue(num8);
        // calculate checksum
        int checksum =(10-(zipSum %10));
        String check = String.valueOf(checksum);
        char checkString = check.charAt(0);
        return ("|" +getBar(num0)+getBar(num1)+getBar(num2)+getBar(num3)+getBar(num4)+getBar(num5)+getBar(num6)+getBar(num7)+getBar(num8)+getBar(checkString)+"|");
    }
    public static String getBar(char num){
    String[] post = {":::||","::|:|","::||:",":|::|",":|:|:",":||::","|:::|","|::|:","|:|::","||:::"};
    if (Character.getNumericValue(num) < 10){
        return post[Character.getNumericValue(num)];
    }
    else {
        return post[10];
    }
    }
}