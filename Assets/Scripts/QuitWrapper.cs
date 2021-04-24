using UnityEngine;
using System.Collections;
using UnityEngine.SceneManagement;

public class QuitWrapper : MonoBehaviour
{
    public GameObject pausePanel;
    public GameObject instructionPanel;

	public void QuitApp() {Application.Quit();}
	public void QuitToStart() {
        Plane.WINNER = 0;
		SceneManager.LoadScene(0); 
		if (Time.timeScale == 0) {
			Time.timeScale = 1;
			pausePanel.SetActive (false);
		}
	}
    public void StartSinglePlayer() { Plane.AI = 1;  SceneManager.LoadScene(1); }
    public void StartMultiPlayer() { Plane.AI = 0;  SceneManager.LoadScene(1); }
    public void ShowInstructions()
    {
        instructionPanel.SetActive(!instructionPanel.activeInHierarchy);
    }
    public void hidePause() { Time.timeScale = 1;  pausePanel.SetActive(false); }
    void Update()
    {
        if(Input.GetKeyDown(KeyCode.Escape))
        {
			if (SceneManager.GetActiveScene().buildIndex != 0) {
				if (Time.timeScale == 1) {
					Time.timeScale = 0;
					pausePanel.SetActive (true);
				} else {
					Time.timeScale = 1;
					pausePanel.SetActive (false);
				}
			}
        }
    }

}
