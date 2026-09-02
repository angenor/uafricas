<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminAnnonces()
const router = useRouter()

const etape = ref(1)
const erreurLocale = ref<string | null>(null)

const form = reactive({
  // Etape 1 : Infos de base
  titre: '',
  description: '',
  type_operation: 'vente',
  condition_article: 'non_applicable',
  prix: null as number | null,
  devise: 'XOF',
  prix_negociable: false,
  quantite: 1,
  // Etape 2 : Categorie
  categorie_id: '',
  etat: 'brouillon',
  // Etape 3 : Contact
  ville: '',
  adresse: '',
  type_contact: 'messagerie_plateforme',
  contact_info: '',
})

const validerEtape1 = () => {
  if (!form.titre.trim()) { erreurLocale.value = 'Le titre est requis'; return false }
  if (!form.description.trim()) { erreurLocale.value = 'La description est requise'; return false }
  erreurLocale.value = null
  return true
}

const suivante = () => {
  if (etape.value === 1 && !validerEtape1()) return
  erreurLocale.value = null
  etape.value++
}

const precedente = () => {
  erreurLocale.value = null
  etape.value--
}

const soumettre = async () => {
  erreurLocale.value = null
  try {
    const body: any = {
      titre: form.titre.trim(),
      description: form.description.trim(),
      type_operation: form.type_operation,
      condition_article: form.condition_article,
      prix_negociable: form.prix_negociable,
      quantite: form.quantite,
      devise: form.devise,
      type_contact: form.type_contact,
      etat: form.etat,
    }
    if (form.prix !== null && form.prix > 0) body.prix = form.prix
    if (form.categorie_id.trim()) body.categorie_id = form.categorie_id.trim()
    if (form.ville.trim()) body.ville = form.ville.trim()
    if (form.adresse.trim()) body.adresse = form.adresse.trim()
    if (form.contact_info.trim()) body.contact_info = form.contact_info.trim()

    await creer(body)
    router.push('/admin/annonces')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle annonce" sous-titre="Creer une annonce sur le Marche Africain">
      <template #actions>
        <NuxtLink to="/admin/annonces" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <!-- Indicateur d'etapes -->
    <ul class="steps steps-horizontal w-full mb-6">
      <li class="step" :class="{ 'step-primary': etape >= 1 }">Infos de base</li>
      <li class="step" :class="{ 'step-primary': etape >= 2 }">Catégorie & État</li>
      <li class="step" :class="{ 'step-primary': etape >= 3 }">Contact & Localisation</li>
    </ul>

    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ erreurLocale || error }}</span>
        </div>

        <!-- Etape 1 : Infos de base -->
        <div v-show="etape === 1" class="space-y-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Titre de l'annonce *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" placeholder="Ex: iPhone 14 Pro Max" required />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description *</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" placeholder="Decrivez votre annonce..." />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type d'operation</span></label>
              <select v-model="form.type_operation" class="select select-bordered">
                <option value="vente">Vente</option>
                <option value="troc">Troc</option>
                <option value="don">Don</option>
                <option value="association">Association</option>
                <option value="opportunite">Opportunite</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Condition</span></label>
              <select v-model="form.condition_article" class="select select-bordered">
                <option value="neuf">Neuf</option>
                <option value="occasion">Occasion</option>
                <option value="reconditionne">Reconditionne</option>
                <option value="non_applicable">Non applicable</option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Prix</span></label>
              <input v-model.number="form.prix" type="number" class="input input-bordered" min="0" step="0.01" placeholder="0" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Devise</span></label>
              <select v-model="form.devise" class="select select-bordered">
                <option value="XOF">XOF (Franc CFA)</option>
                <option value="XAF">XAF (Franc CFA Central)</option>
                <option value="EUR">EUR</option>
                <option value="USD">USD</option>
                <option value="GBP">GBP</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Quantite</span></label>
              <input v-model.number="form.quantite" type="number" class="input input-bordered" min="1" />
            </div>
          </div>

          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3">
              <input v-model="form.prix_negociable" type="checkbox" class="checkbox checkbox-primary" />
              <span class="label-text">Prix negociable</span>
            </label>
          </div>
        </div>

        <!-- Etape 2 : Categorie & Etat -->
        <div v-show="etape === 2" class="space-y-4">
          <div class="form-control">
            <label class="label"><span class="label-text">ID Catégorie (UUID)</span></label>
            <input v-model="form.categorie_id" type="text" class="input input-bordered" placeholder="Laisser vide si aucune catégorie" />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">État initial</span></label>
            <select v-model="form.etat" class="select select-bordered">
              <option value="brouillon">Brouillon</option>
              <option value="en_attente">En attente de validation</option>
              <option value="publiee">Publiée</option>
            </select>
          </div>

          <div class="alert alert-info">
            <font-awesome-icon icon="circle-info" />
            <span>Les pays cibles et les medias pourront etre ajoutes apres la creation de l'annonce, sur la page d'edition.</span>
          </div>
        </div>

        <!-- Etape 3 : Contact -->
        <div v-show="etape === 3" class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Ville</span></label>
              <input v-model="form.ville" type="text" class="input input-bordered" placeholder="Ex: Dakar" />
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Adresse</span></label>
              <input v-model="form.adresse" type="text" class="input input-bordered" placeholder="Adresse complete" />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Type de contact</span></label>
              <select v-model="form.type_contact" class="select select-bordered">
                <option value="messagerie_plateforme">Messagerie plateforme</option>
                <option value="email">Email</option>
                <option value="telephone">Téléphone</option>
              </select>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Info de contact</span></label>
              <input v-model="form.contact_info" type="text" class="input input-bordered" placeholder="Email ou telephone" />
            </div>
          </div>
        </div>

        <!-- Navigation etapes -->
        <div class="flex items-center justify-between pt-6 border-t mt-6">
          <button v-if="etape > 1" class="btn btn-ghost" @click="precedente">
            <font-awesome-icon icon="arrow-left" class="mr-1" /> Précédent
          </button>
          <div v-else />

          <div class="flex gap-2">
            <NuxtLink to="/admin/annonces" class="btn btn-ghost">Annuler</NuxtLink>
            <button v-if="etape < 3" class="btn btn-primary" @click="suivante">
              Suivant <font-awesome-icon icon="arrow-right" class="ml-1" />
            </button>
            <button v-else class="btn btn-primary" :class="{ loading }" :disabled="loading" @click="soumettre">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Creer l'annonce
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
