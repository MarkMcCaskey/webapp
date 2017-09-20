function validatePassword() {
  if(password.value != confirm_password.value){
    confirm_password.setCustomValidity("Passwords don't match");
  } else {
    confirm_password.setCustomValidity('');
  }
}

function passwordValidation() {
  var password = document.getElementById("password");
  var confirm_password = document.getElementById("confirm_password");
  password.onchange = validatePassword;
  confirm_password.onkeyup = validatePassword;
}
