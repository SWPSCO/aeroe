## New User Flow

1.  User downloads and opens the app.
2.  User is presented with Terms of Use and must click "Accept" to continue.
3.  User is presented with a Privacy Policy and must click "Accept" to continue.
4.  User is presented with a "Create Local Password" prompt.
    -   This password encrypts the user's local wallet data.
    -   The UI will have two fields: "Password" and "Confirm Password".
    -   A strong warning will be displayed: **"This password cannot be recovered. If you forget it, you will need your 24-word recovery phrase to restore access to your wallet."**
5.  User enters and confirms their password.
6.  User is presented with two buttons: "Import Wallet" or "Create Wallet".

### Import Wallet Flow

1.  User is presented with 24 input boxes for their 24-word recovery phrase.
2.  User enters the 24 words.
3.  User clicks the "Import" button.
4.  The application validates the recovery phrase.
    -   **On Success:** A loading screen is shown, and the user is taken to the Wallet Dashboard.
    -   **On Failure (Invalid Phrase):** An error message is displayed (e.g., "Invalid recovery phrase. Please check your words and try again."). The user remains on the import screen to correct their entry.
5.  The user is sent to the Wallet Dashboard.

### Create Wallet Flow

1.  The application generates a new, cryptographically secure 24-word recovery phrase.
2.  The phrase is displayed to the user with prominent warnings:
    -   "This is your recovery phrase. Write it down in the correct order and store it in a safe, offline location."
    -   "Never share this phrase with anyone. Anyone with this phrase can access your funds."
3.  The user clicks a button like "I Have Saved My Phrase" to proceed to a verification step.
4.  The application prompts the user to verify their phrase to ensure it was backed up correctly.
    -   Example: "To confirm, please enter word #3, #8, and #21 from your phrase."
5.  The user enters the requested words.
    -   **On Success:** The user is taken to the Wallet Dashboard.
    -   **On Failure:** An error message is shown, and the user is given the option to try again or go back to view their phrase.
6.  The user is sent to the Wallet Dashboard.

## Existing User Flow

1.  User opens the app.
2.  User is presented with an "Enter Local Password" prompt to decrypt their wallet data.
3.  User enters their password.
    -   **On Success:** The user is taken directly to the Wallet Dashboard.
    -   **On Failure (Incorrect Password):** An error message is displayed. The user is allowed to try again.

## Dashboard

### Wallet Page

1.  Balance display
2.  "Send" button/feature
3.  "Receive" button/feature
4.  Transaction history list

### Mining Page

1.  Login for Nockpool
2.  Button for solo mining

### Explorer Page

1.  Block explorer (TBD)
