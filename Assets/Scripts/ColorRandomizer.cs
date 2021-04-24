using UnityEngine;
using System.Collections;

public class ColorRandomizer : MonoBehaviour {

	public static Color UColor;
	public static Color UColor2;
	private float Hue=56;

	// Use this for initialization
	void Start () {
		Hue = Random.Range(0,360);
		Hue /= 360;
		UColor = Color.HSVToRGB (Hue, 1,1);
		UColor2 = Color.HSVToRGB ((Hue+0.5f)%1, 1,1);
		Resources.Load<Material> ("Materials/Brown").color = UColor;
		Resources.Load<Material> ("Materials/Gray").color = UColor2;
	}
}
