using UnityEngine;
using System.Collections;
using UnityEngine.UI;

public class WinnerDetection : MonoBehaviour
{

	// Use this for initialization
	void Start ()
    {
        Text t = transform.GetChild(0).GetComponent<Text>();
		if (Plane.WINNER == 0) {
			t.text = "You should not be here.";
		} else if (Plane.WINNER == 1) {
			t.text = t.text.Substring (0, 7) + Plane.WINNER + t.text.Substring (6);
			t.color=Resources.Load<Material> ("Materials/Brown").color;
		} else if (Plane.WINNER == 2) {
			t.text = t.text.Substring (0, 7) + Plane.WINNER + t.text.Substring (6);
			t.color=Resources.Load<Material> ("Materials/Gray").color;
		} else {
			t.text = "It's a draw!";
		}
	}
	
	// Update is called once per frame
	void Update () {
	
	}
}
