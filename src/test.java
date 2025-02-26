// copyright leckmich 2025

import bla.fasel;

/* unrelated block comment */

/**
 * Hero is the main entity we'll be using to . . .
 *
 * Please see the {@link com.baeldung.javadoc.Person} class for true identity
 * @author Captain America
 *
 */
public class SuperHero extends Person {


	/**
     * javadoc here
     */
	private String coolerboi;
	
	/**
	 * Returns an Image object that can then be painted on the screen.
	 * The url argument must specify an absolute. The name
	 * argument is a specifier that is relative to the url argument.
	 * 
	 * This method always returns immediately, whether or not the
	 * image exists. When this applet ttempts to draw the image on
	 * the screen, the data will be loaded. The graphics primitives
	 * that draw the image will incrementally paint on the screen.
	 *
	 * @param  url  an absolute URL giving the base location of the image
	 * @param  name the location of the image, relative to the url argument
	 * @return      the image at the specified URL
	 * @see         Image
	 */
	public Image getImage(URL url, String name) {
		try {
			return getImage(new URL(url, name));
		} catch (MalformedURLException e) {
			return null;
		}
	}

	/**
	 * schau mal mama, eine Subklasse!
	 */
	public class stumpfIstTrumpf{
		/**
		* langsam wirds albern
		*/
		private String unterfeld;
	}
}
